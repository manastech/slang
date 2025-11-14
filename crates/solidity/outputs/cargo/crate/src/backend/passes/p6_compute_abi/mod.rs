use crate::backend::abi;
use crate::backend::context::SemanticAnalysis;
use crate::backend::ir::ir2_flat_contracts as output_ir;

pub fn run(semantic: &SemanticAnalysis) -> Vec<abi::ContractAbi> {
    let mut contracts = Vec::new();
    for (file_id, source_unit) in &semantic.files {
        for member in &source_unit.members {
            let output_ir::SourceUnitMember::ContractDefinition(contract_definition) = member
            else {
                continue;
            };
            if contract_definition.abstract_keyword {
                continue;
            }
            let name = contract_definition.name.unparse();
            let functions = get_functions_for_contract(contract_definition);
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
    contract_definition: &output_ir::ContractDefinition,
) -> Vec<abi::FunctionAbi> {
    let mut functions = Vec::new();
    for member in &contract_definition.members {
        match member {
            output_ir::ContractMember::FunctionDefinition(function_definition) => {
                if !matches!(
                    function_definition.visibility,
                    output_ir::FunctionVisibility::Public | output_ir::FunctionVisibility::External
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
                    output_ir::FunctionMutability::Payable => abi::FunctionAbiMutability::Payable,
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
