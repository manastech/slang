#![allow(missing_docs)]

use std::collections::HashMap;

use binder::Binder;
use types::TypeRegistry;

use self::abi::ContractAbi;
use crate::backend::ir::ir2_flat_contracts as output_ir;
use crate::compilation::CompilationUnit;

pub mod abi;
pub mod binder;
pub mod built_ins;
pub mod ir;
pub mod passes;
pub mod types;

pub type BinderOutput = passes::p5_resolve_references::Output;

pub fn build_binder_output(compilation_unit: CompilationUnit) -> BinderOutput {
    let data = passes::p0_build_ast::run(compilation_unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_linearise_contracts::run(data);
    let data = passes::p4_type_definitions::run(data);
    passes::p5_resolve_references::run(data)
}

pub struct BackendContext {
    compilation_unit: CompilationUnit,
    files: HashMap<String, output_ir::SourceUnit>,
    binder: Binder,
    types: TypeRegistry,
}

impl BackendContext {
    pub fn build(compilation_unit: CompilationUnit) -> Self {
        let output = build_binder_output(compilation_unit);
        Self {
            compilation_unit: output.compilation_unit,
            files: output.files,
            binder: output.binder,
            types: output.types,
        }
    }

    pub fn compilation_unit(&self) -> &CompilationUnit {
        &self.compilation_unit
    }

    pub fn get_file_ir(&self, file_id: &str) -> Option<&output_ir::SourceUnit> {
        self.files.get(file_id)
    }

    pub fn binder(&self) -> &Binder {
        &self.binder
    }

    pub fn types(&self) -> &TypeRegistry {
        &self.types
    }

    pub fn get_contracts(&self) -> Vec<ContractAbi> {
        let mut contracts = Vec::new();
        for (file_id, source_unit) in &self.files {
            for member in &source_unit.members {
                let output_ir::SourceUnitMember::ContractDefinition(contract_definition) = member
                else {
                    continue;
                };
                if contract_definition.abstract_keyword {
                    continue;
                }
                let name = contract_definition.name.unparse();
                let functions = self.get_functions_for_contract(contract_definition);
                let contract = abi::ContractAbi {
                    node_id: contract_definition.node_id,
                    name,
                    file_id: file_id.to_string(),
                    functions,
                    storage_layout: Vec::new(), // TODO
                };
                contracts.push(contract);
            }
        }
        contracts
    }

    fn get_functions_for_contract(
        &self,
        contract_definition: &output_ir::ContractDefinition,
    ) -> Vec<abi::FunctionAbi> {
        let mut functions = Vec::new();
        for member in &contract_definition.members {
            match member {
                output_ir::ContractMember::FunctionDefinition(function_definition) => {
                    if !matches!(
                        function_definition.visibility,
                        output_ir::FunctionVisibility::Public
                            | output_ir::FunctionVisibility::External
                    ) {
                        continue;
                    }

                    let kind = match function_definition.kind {
                        output_ir::FunctionKind::Regular => abi::FunctionAbiType::Function,
                        output_ir::FunctionKind::Constructor => abi::FunctionAbiType::Constructor,
                        output_ir::FunctionKind::Unnamed | output_ir::FunctionKind::Fallback => {
                            abi::FunctionAbiType::Fallback
                        }
                        output_ir::FunctionKind::Receive => abi::FunctionAbiType::Receive,
                        output_ir::FunctionKind::Modifier => continue,
                    };
                    let state_mutability = match function_definition.mutability {
                        output_ir::FunctionMutability::Pure => abi::FunctionAbiMutability::Pure,
                        output_ir::FunctionMutability::View => abi::FunctionAbiMutability::View,
                        output_ir::FunctionMutability::NonPayable => {
                            abi::FunctionAbiMutability::NonPayable
                        }
                        output_ir::FunctionMutability::Payable => {
                            abi::FunctionAbiMutability::Payable
                        }
                    };
                    let function = abi::FunctionAbi {
                        node_id: function_definition.node_id,
                        name: function_definition.name.as_ref().map(|name| name.unparse()),
                        kind,
                        inputs: Vec::new(),  // TODO
                        outputs: Vec::new(), // TODO
                        state_mutability,
                    };
                    functions.push(function);
                }
                output_ir::ContractMember::StateVariableDefinition(state_variable_definition) => {
                    if !matches!(
                        state_variable_definition.visibility,
                        output_ir::StateVariableVisibility::Public
                    ) {
                        continue;
                    }
                    let function = abi::FunctionAbi {
                        node_id: state_variable_definition.node_id,
                        name: Some(state_variable_definition.name.unparse()),
                        kind: abi::FunctionAbiType::Function,
                        inputs: Vec::new(),  // TODO
                        outputs: Vec::new(), // TODO
                        state_mutability: abi::FunctionAbiMutability::View,
                    };
                    functions.push(function);
                }
                _ => continue,
            }
        }
        functions
    }
}
