use std::collections::HashMap;

use metaslang_cst::nodes::NodeId;

use super::p1_flatten_contracts::Output as Input;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::l2_flat_contracts::{visitor::Visitor, SourceUnit};

pub struct Output {
    pub files: HashMap<String, SourceUnit>,
    pub definitions: HashMap<NodeId, Definition>,
}

pub fn run(input: &Input) -> Output {
    let files = input.files.clone();
    let mut pass = Pass::new();
    for source_unit in files.values() {
        input_ir::visitor::accept_source_unit(source_unit, &mut pass);
    }
    let definitions = pass.definitions;

    Output { files, definitions }
}

pub struct Definition {
    pub name_node_id: NodeId,
    pub definiens_node_id: NodeId,
}

struct Pass {
    definitions: HashMap<NodeId, Definition>,
}

impl Pass {
    fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    fn insert_definition(&mut self, definiens_node_id: NodeId, name_node_id: NodeId) {
        let definition = Definition {
            name_node_id,
            definiens_node_id,
        };
        self.definitions.insert(definiens_node_id, definition);
    }
}

impl Visitor for Pass {
    fn enter_path_import(&mut self, node: &input_ir::PathImport) -> bool {
        if let Some(alias) = &node.alias {
            self.insert_definition(node.node_id, alias.identifier.id());
        }
        false
    }

    fn enter_named_import(&mut self, node: &input_ir::NamedImport) -> bool {
        self.insert_definition(node.node_id, node.alias.identifier.id());
        false
    }

    fn enter_import_deconstruction_symbol(
        &mut self,
        node: &input_ir::ImportDeconstructionSymbol,
    ) -> bool {
        if let Some(alias) = &node.alias {
            self.insert_definition(node.node_id, alias.identifier.id());
        } else {
            self.insert_definition(node.node_id, node.name.id());
        }
        false
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        true
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        true
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        true
    }

    fn enter_mapping_key(&mut self, node: &input_ir::MappingKey) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name.id());
        }
        false
    }

    fn enter_mapping_value(&mut self, node: &input_ir::MappingValue) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name.id());
        }
        false
    }

    fn enter_parameter(&mut self, node: &input_ir::Parameter) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name.id());
        }
        false
    }

    fn enter_function_definition(&mut self, node: &input_ir::FunctionDefinition) -> bool {
        if let input_ir::FunctionName::Identifier(name) = &node.name {
            self.insert_definition(node.node_id, name.id());
        }
        true
    }

    fn enter_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        true
    }

    fn enter_variable_declaration_statement(
        &mut self,
        node: &input_ir::VariableDeclarationStatement,
    ) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        false
    }

    fn enter_tuple_deconstruction_element(
        &mut self,
        node: &input_ir::TupleDeconstructionElement,
    ) -> bool {
        match &node.member {
            Some(input_ir::TupleMember::TypedTupleMember(member)) => {
                self.insert_definition(member.node_id, member.name.id());
            }
            Some(input_ir::TupleMember::UntypedTupleMember(member)) => {
                self.insert_definition(member.node_id, member.name.id());
            }
            None => {}
        }
        false
    }

    fn enter_state_variable_definition(
        &mut self,
        node: &input_ir::StateVariableDefinition,
    ) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        false
    }

    fn enter_enum_definition(&mut self, node: &input_ir::EnumDefinition) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        true
    }

    fn enter_enum_members(&mut self, items: &input_ir::EnumMembers) -> bool {
        for item in items {
            self.insert_definition(item.id(), item.id());
        }
        false
    }

    fn enter_struct_definition(&mut self, node: &input_ir::StructDefinition) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        true
    }

    fn enter_struct_member(&mut self, node: &input_ir::StructMember) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        false
    }

    fn enter_event_definition(&mut self, node: &input_ir::EventDefinition) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        true
    }

    fn enter_event_parameter(&mut self, node: &input_ir::EventParameter) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name.id());
        }
        false
    }

    fn enter_error_definition(&mut self, node: &input_ir::ErrorDefinition) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        true
    }

    fn enter_error_parameter(&mut self, node: &input_ir::ErrorParameter) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name.id());
        }
        false
    }

    fn enter_constant_definition(&mut self, node: &input_ir::ConstantDefinition) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        false
    }

    fn enter_user_defined_value_type_definition(
        &mut self,
        node: &input_ir::UserDefinedValueTypeDefinition,
    ) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        false
    }

    fn enter_yul_variable_declaration_statement(
        &mut self,
        node: &input_ir::YulVariableDeclarationStatement,
    ) -> bool {
        for variable in &node.variables {
            self.insert_definition(variable.id(), variable.id());
        }
        false
    }

    fn enter_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) -> bool {
        self.insert_definition(node.node_id, node.name.id());
        true
    }

    fn enter_yul_parameters_declaration(&mut self, node: &input_ir::YulParametersDeclaration) -> bool {
        for parameter in &node.parameters {
            self.insert_definition(parameter.id(), parameter.id());
        }
        false
    }

    fn enter_yul_returns_declaration(&mut self, node: &input_ir::YulReturnsDeclaration) -> bool {
        for parameter in &node.variables {
            self.insert_definition(parameter.id(), parameter.id());
        }
        false
    }
}
