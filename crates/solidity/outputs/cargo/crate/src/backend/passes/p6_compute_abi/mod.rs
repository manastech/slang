use std::ops::Div;

use crate::backend::abi::{
    ContractAbi, FunctionAbi, FunctionAbiMutability, FunctionAbiType, FunctionInputOutput, Slot,
};
use crate::backend::binder::Definition;
use crate::backend::context::semantic::SLOT_SIZE;
use crate::backend::context::SemanticAnalysis;
use crate::backend::ir::ir2_flat_contracts as output_ir;
use crate::backend::types::{Type, TypeId};

pub fn run(semantic: &SemanticAnalysis) -> Vec<ContractAbi> {
    let mut pass = Pass::new(semantic);
    for (file_id, source_unit) in &semantic.files {
        pass.visit_file(file_id, source_unit);
    }
    pass.contracts
}

struct Pass<'a> {
    semantic: &'a SemanticAnalysis,
    contracts: Vec<ContractAbi>,
}

impl<'a> Pass<'a> {
    fn new(semantic: &'a SemanticAnalysis) -> Self {
        Self {
            semantic,
            contracts: Vec::new(),
        }
    }

    fn visit_file(&mut self, file_id: &str, source_unit: &output_ir::SourceUnit) {
        for member in &source_unit.members {
            if let Some(contract) = self.visit_source_unit_member(file_id, member) {
                self.contracts.push(contract);
            }
        }
    }

    fn visit_source_unit_member(
        &self,
        file_id: &str,
        member: &output_ir::SourceUnitMember,
    ) -> Option<ContractAbi> {
        let output_ir::SourceUnitMember::ContractDefinition(contract_definition) = member else {
            return None;
        };
        if contract_definition.abstract_keyword {
            return None;
        }

        let name = contract_definition.name.unparse();
        let functions = self.visit_contract_members(&contract_definition.members);
        let storage_layout = self.compute_storage_layout(&contract_definition.members)?;
        Some(ContractAbi {
            node_id: contract_definition.node_id,
            name,
            file_id: file_id.to_string(),
            functions,
            storage_layout,
        })
    }

    fn visit_contract_members(&self, members: &output_ir::ContractMembers) -> Vec<FunctionAbi> {
        let mut functions = Vec::new();
        for member in members {
            let function = match member {
                output_ir::ContractMember::FunctionDefinition(function_definition) => {
                    self.visit_function_definition(function_definition)
                }
                output_ir::ContractMember::StateVariableDefinition(state_variable_definition) => {
                    self.visit_state_variable_definition(state_variable_definition)
                }
                _ => continue,
            };
            if let Some(function) = function {
                functions.push(function);
            }
        }
        functions
    }

    fn compute_storage_layout(&self, members: &output_ir::ContractMembers) -> Option<Vec<Slot>> {
        let mut storage_layout = Vec::new();
        let mut ptr: usize = 0;
        for member in members {
            let output_ir::ContractMember::StateVariableDefinition(state_variable_definition) =
                member
            else {
                continue;
            };
            let node_id = state_variable_definition.node_id;
            // skip constants and immutable variables, since they are not placed in storage
            // TODO: also, transient storage is laid out separately and we need
            // to support that as well
            if !matches!(
                state_variable_definition.mutability,
                output_ir::StateVariableMutability::Mutable
            ) {
                continue;
            }

            let variable_type_id = self.semantic.binder.node_typing(node_id).as_type_id()?;
            let variable_size = self.semantic.storage_size_of_type_id(variable_type_id)?;

            // check if we can pack the variable in the previous slot
            let remaining_bytes = SLOT_SIZE - (ptr % SLOT_SIZE);
            if variable_size >= remaining_bytes {
                ptr += remaining_bytes;
            }

            let label = state_variable_definition.name.unparse();
            let slot = ptr.div(SLOT_SIZE);
            let offset = ptr % SLOT_SIZE;
            let r#type = self.semantic.type_canonical_name(variable_type_id);
            storage_layout.push(Slot {
                node_id,
                label,
                slot,
                offset,
                r#type,
            });

            // ready pointer for the next variable
            ptr += variable_size;
        }
        Some(storage_layout)
    }

    fn visit_function_definition(
        &self,
        function_definition: &output_ir::FunctionDefinition,
    ) -> Option<FunctionAbi> {
        if !matches!(
            function_definition.visibility,
            output_ir::FunctionVisibility::Public | output_ir::FunctionVisibility::External
        ) {
            return None;
        }

        let kind = match function_definition.kind {
            output_ir::FunctionKind::Regular => FunctionAbiType::Function,
            output_ir::FunctionKind::Constructor => FunctionAbiType::Constructor,
            output_ir::FunctionKind::Unnamed | output_ir::FunctionKind::Fallback => {
                FunctionAbiType::Fallback
            }
            output_ir::FunctionKind::Receive => FunctionAbiType::Receive,
            output_ir::FunctionKind::Modifier => return None,
        };
        let state_mutability = match function_definition.mutability {
            output_ir::FunctionMutability::Pure => FunctionAbiMutability::Pure,
            output_ir::FunctionMutability::View => FunctionAbiMutability::View,
            output_ir::FunctionMutability::NonPayable => FunctionAbiMutability::NonPayable,
            output_ir::FunctionMutability::Payable => FunctionAbiMutability::Payable,
        };

        // If any parameter is not properly typed we bail out with `None`
        let inputs = self.visit_function_parameters(&function_definition.parameters)?;
        let outputs = if let Some(returns) = &function_definition.returns {
            self.visit_function_parameters(returns)?
        } else {
            Vec::new()
        };

        Some(FunctionAbi {
            node_id: function_definition.node_id,
            name: function_definition.name.as_ref().map(|name| name.unparse()),
            kind,
            inputs,
            outputs,
            state_mutability,
        })
    }

    fn visit_function_parameters(
        &self,
        parameters: &output_ir::Parameters,
    ) -> Option<Vec<FunctionInputOutput>> {
        let mut result = Vec::new();
        for parameter in parameters {
            let node_id = parameter.node_id;
            let name = parameter.name.as_ref().map(|name| name.unparse());
            // Bail out with `None` if any of the parameters fails typing
            let type_id = self.semantic.binder.node_typing(node_id).as_type_id()?;
            let r#type = self.semantic.type_canonical_name(type_id);
            result.push(FunctionInputOutput {
                node_id: Some(node_id),
                name,
                r#type,
            });
        }
        Some(result)
    }

    fn visit_state_variable_definition(
        &self,
        state_variable_definition: &output_ir::StateVariableDefinition,
    ) -> Option<FunctionAbi> {
        if !matches!(
            state_variable_definition.visibility,
            output_ir::StateVariableVisibility::Public
        ) {
            return None;
        }
        let Some(Definition::StateVariable(definition)) = self
            .semantic
            .binder
            .find_definition_by_id(state_variable_definition.node_id)
        else {
            return None;
        };
        let (inputs, outputs) =
            self.extract_function_type_parameters(definition.getter_type_id?)?;

        Some(FunctionAbi {
            node_id: state_variable_definition.node_id,
            name: Some(state_variable_definition.name.unparse()),
            kind: FunctionAbiType::Function,
            inputs,
            outputs,
            state_mutability: FunctionAbiMutability::View,
        })
    }

    fn extract_function_type_parameters(
        &self,
        type_id: TypeId,
    ) -> Option<(Vec<FunctionInputOutput>, Vec<FunctionInputOutput>)> {
        let Type::Function(function_type) = self.semantic.types.get_type_by_id(type_id) else {
            return None;
        };
        let inputs = function_type
            .parameter_types
            .iter()
            .map(|parameter_type_id| FunctionInputOutput {
                node_id: None,
                name: None,
                r#type: self.semantic.type_canonical_name(*parameter_type_id),
            })
            .collect();
        let outputs = vec![FunctionInputOutput {
            node_id: None,
            name: None,
            r#type: self.semantic.type_canonical_name(function_type.return_type),
        }];
        Some((inputs, outputs))
    }
}
