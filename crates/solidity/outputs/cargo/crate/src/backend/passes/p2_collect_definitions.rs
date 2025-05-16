use std::collections::HashMap;
use std::rc::Rc;

use super::p1_flatten_contracts::Output as Input;
use crate::backend::binder::{Binder, ScopeId};
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::cst::{NodeId, TerminalNode};

pub struct Output {
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let mut pass = Pass::new();
    for (file_id, source_unit) in &files {
        pass.visit_file(file_id, source_unit);
    }
    let binder = pass.binder;

    Output { files, binder }
}

struct Pass {
    binder: Binder,
    scope_stack: Vec<ScopeId>,
}

impl Pass {
    fn new() -> Self {
        Self {
            binder: Binder::default(),
            scope_stack: Vec::new(),
        }
    }

    fn visit_file(&mut self, _file_id: &str, source_unit: &input_ir::SourceUnit) {
        input_ir::visitor::accept_source_unit(source_unit, self);
        assert!(self.scope_stack.is_empty());
    }

    fn push_scope(&mut self, node_id: NodeId) {
        let parent_scope_id = self.scope_stack.last().copied();
        let new_scope_id = self.binder.insert_scope_at(node_id, parent_scope_id);
        self.scope_stack.push(new_scope_id);
    }

    fn pop_scope(&mut self) -> ScopeId {
        let Some(scope) = self.scope_stack.pop() else {
            unreachable!("scope stack empty");
        };
        scope
    }

    fn scope_node(&mut self, node_id: NodeId) {
        let Some(scope_id) = self.scope_stack.last() else {
            unreachable!("attempt to scope a node with an empty scope stack");
        };
        self.binder.link_node_to_scope(node_id, *scope_id);
    }

    fn insert_definition(&mut self, definiens_node_id: NodeId, name_node: &Rc<TerminalNode>) {
        let Some(scope_id) = self.scope_stack.last() else {
            unreachable!("attempt to insert a definition with an empty scope stack");
        };
        let name_node_id = name_node.id();
        let name = name_node.unparse();
        self.binder
            .insert_definition_at_scope(*scope_id, name_node_id, name, definiens_node_id);
    }
}

impl Visitor for Pass {
    fn enter_source_unit(&mut self, node: &input_ir::SourceUnit) -> bool {
        assert!(self.scope_stack.is_empty());

        self.push_scope(node.node_id);
        true
    }

    fn leave_source_unit(&mut self, _node: &input_ir::SourceUnit) {
        self.pop_scope();
    }

    fn enter_path_import(&mut self, node: &input_ir::PathImport) -> bool {
        if let Some(alias) = &node.alias {
            self.insert_definition(node.node_id, &alias.identifier);
        }
        false
    }

    fn enter_named_import(&mut self, node: &input_ir::NamedImport) -> bool {
        self.insert_definition(node.node_id, &node.alias.identifier);
        false
    }

    fn enter_import_deconstruction_symbol(
        &mut self,
        node: &input_ir::ImportDeconstructionSymbol,
    ) -> bool {
        if let Some(alias) = &node.alias {
            self.insert_definition(node.node_id, &alias.identifier);
        } else {
            self.insert_definition(node.node_id, &node.name);
        }
        true
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        self.push_scope(node.node_id);
        true
    }

    fn leave_contract_definition(&mut self, _node: &input_ir::ContractDefinition) {
        self.pop_scope();
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        self.push_scope(node.node_id);
        true
    }

    fn leave_interface_definition(&mut self, _node: &input_ir::InterfaceDefinition) {
        self.pop_scope();
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        self.push_scope(node.node_id);
        true
    }

    fn leave_library_definition(&mut self, _node: &input_ir::LibraryDefinition) {
        self.pop_scope();
    }

    fn enter_function_definition(&mut self, node: &input_ir::FunctionDefinition) -> bool {
        if let input_ir::FunctionName::Identifier(name) = &node.name {
            self.insert_definition(node.node_id, name);
        }
        self.push_scope(node.node_id);
        true
    }

    fn leave_function_definition(&mut self, _node: &input_ir::FunctionDefinition) {
        self.pop_scope();
    }

    fn enter_parameter(&mut self, node: &input_ir::Parameter) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name);
        }
        true
    }

    fn enter_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        self.push_scope(node.node_id);
        true
    }

    fn leave_modifier_definition(&mut self, _node: &input_ir::ModifierDefinition) {
        self.pop_scope();
    }

    fn enter_variable_declaration_statement(
        &mut self,
        node: &input_ir::VariableDeclarationStatement,
    ) -> bool {
        self.insert_definition(node.node_id, &node.name);
        true
    }

    fn enter_tuple_deconstruction_element(
        &mut self,
        node: &input_ir::TupleDeconstructionElement,
    ) -> bool {
        match &node.member {
            Some(input_ir::TupleMember::TypedTupleMember(member)) => {
                self.insert_definition(member.node_id, &member.name);
            }
            Some(input_ir::TupleMember::UntypedTupleMember(member)) => {
                self.insert_definition(member.node_id, &member.name);
            }
            None => {}
        }
        true
    }

    fn enter_state_variable_definition(
        &mut self,
        node: &input_ir::StateVariableDefinition,
    ) -> bool {
        self.insert_definition(node.node_id, &node.name);
        // need to visit to gather possible mapping key/value names
        true
    }

    fn enter_enum_definition(&mut self, node: &input_ir::EnumDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        self.push_scope(node.node_id);
        true
    }

    fn leave_enum_definition(&mut self, _node: &input_ir::EnumDefinition) {
        self.pop_scope();
    }

    fn enter_enum_members(&mut self, items: &input_ir::EnumMembers) -> bool {
        for item in items {
            self.insert_definition(item.id(), item);
        }
        false
    }

    fn enter_struct_definition(&mut self, node: &input_ir::StructDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        self.push_scope(node.node_id);
        true
    }

    fn leave_struct_definition(&mut self, _node: &input_ir::StructDefinition) {
        self.pop_scope();
    }

    fn enter_struct_member(&mut self, node: &input_ir::StructMember) -> bool {
        self.insert_definition(node.node_id, &node.name);
        true
    }

    fn enter_mapping_key(&mut self, node: &input_ir::MappingKey) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name);
        }
        true
    }

    fn enter_mapping_value(&mut self, node: &input_ir::MappingValue) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name);
        }
        true
    }

    fn enter_event_definition(&mut self, node: &input_ir::EventDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        self.push_scope(node.node_id);
        true
    }

    fn leave_event_definition(&mut self, _node: &input_ir::EventDefinition) {
        self.pop_scope();
    }

    fn enter_event_parameter(&mut self, node: &input_ir::EventParameter) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name);
        }
        true
    }

    fn enter_error_definition(&mut self, node: &input_ir::ErrorDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        self.push_scope(node.node_id);
        true
    }

    fn leave_error_definition(&mut self, _node: &input_ir::ErrorDefinition) {
        self.pop_scope();
    }

    fn enter_error_parameter(&mut self, node: &input_ir::ErrorParameter) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name);
        }
        true
    }

    fn enter_constant_definition(&mut self, node: &input_ir::ConstantDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        true
    }

    fn enter_user_defined_value_type_definition(
        &mut self,
        node: &input_ir::UserDefinedValueTypeDefinition,
    ) -> bool {
        self.insert_definition(node.node_id, &node.name);
        true
    }

    fn enter_yul_variable_declaration_statement(
        &mut self,
        node: &input_ir::YulVariableDeclarationStatement,
    ) -> bool {
        for variable in &node.variables {
            self.insert_definition(variable.id(), variable);
        }
        true
    }

    fn enter_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        self.push_scope(node.node_id);
        true
    }

    fn leave_yul_function_definition(&mut self, _node: &input_ir::YulFunctionDefinition) {
        self.pop_scope();
    }

    fn enter_yul_parameters_declaration(
        &mut self,
        node: &input_ir::YulParametersDeclaration,
    ) -> bool {
        for parameter in &node.parameters {
            self.insert_definition(parameter.id(), parameter);
        }
        false
    }

    fn enter_yul_returns_declaration(&mut self, node: &input_ir::YulReturnsDeclaration) -> bool {
        for parameter in &node.variables {
            self.insert_definition(parameter.id(), parameter);
        }
        false
    }

    // Nodes where references that need to be resolved using lexical scope will appear

    fn leave_import_deconstruction_symbol(&mut self, node: &input_ir::ImportDeconstructionSymbol) {
        self.scope_node(node.node_id);
    }

    fn leave_identifier_path(&mut self, items: &input_ir::IdentifierPath) {
        // we only need to scope the first item's node, the rest will resolve using scope chaining
        if let Some(first_item) = items.first() {
            self.scope_node(first_item.id());
        }
    }

    fn leave_catch_clause_error(&mut self, node: &input_ir::CatchClauseError) {
        self.scope_node(node.node_id);
    }

    fn enter_expression(&mut self, node: &input_ir::Expression) -> bool {
        match node {
            input_ir::Expression::Identifier(identifier) => {
                self.scope_node(identifier.id());
                false
            }
            _ => true,
        }
    }

    fn leave_named_argument(&mut self, node: &input_ir::NamedArgument) {
        self.scope_node(node.node_id);
    }

    fn leave_yul_path(&mut self, items: &input_ir::YulPath) {
        // we only need to scope the first item's node, the rest will resolve using scope chaining
        if let Some(first_item) = items.first() {
            self.scope_node(first_item.id());
        }
    }
}
