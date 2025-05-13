use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use super::p1_flatten_contracts::Output as Input;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::l2_flat_contracts::{visitor::Visitor, SourceUnit};
use crate::cst::TerminalNode;

pub struct Output {
    pub files: HashMap<String, SourceUnit>,
    pub definitions: HashMap<NodeId, Definition>,
}

pub fn run(input: &Input) -> Output {
    let files = input.files.clone();
    let mut pass = Pass::new();
    for (file_id, source_unit) in files.iter() {
        pass.visit_file(file_id, source_unit);
    }
    let definitions = pass.definitions;

    Output { files, definitions }
}

pub struct Definition {
    pub name: String,
    pub name_node_id: NodeId,
    pub definiens_node_id: NodeId,
}

struct Scope {
    #[allow(dead_code)]
    parent: Option<Rc<RefCell<Scope>>>,
    definitions: HashMap<String, NodeId>,
}

impl Scope {
    fn new_root() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent: None,
            definitions: HashMap::new(),
        }))
    }

    fn new(parent: &Rc<RefCell<Scope>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent: Some(Rc::clone(parent)),
            definitions: HashMap::new(),
        }))
    }
}

struct Pass {
    definitions: HashMap<NodeId, Definition>,
    scopes: HashMap<NodeId, Rc<RefCell<Scope>>>,
    scope_stack: Vec<Rc<RefCell<Scope>>>,
}

impl Pass {
    fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            scopes: HashMap::new(),
            scope_stack: Vec::new(),
        }
    }

    fn push_scope(&mut self, node_id: NodeId) {
        let new_scope = if let Some(parent_scope) = self.scope_stack.last() {
            Scope::new(parent_scope)
        } else {
            Scope::new_root()
        };
        self.scopes.insert(node_id, Rc::clone(&new_scope));
        self.scope_stack.push(new_scope);
    }

    fn pop_scope(&mut self) -> Rc<RefCell<Scope>> {
        let Some(scope) = self.scope_stack.pop() else {
            unreachable!("scope stack empty");
        };
        scope
    }

    fn visit_file(&mut self, _file_id: &str, source_unit: &input_ir::SourceUnit) {
        input_ir::visitor::accept_source_unit(source_unit, self);
        assert!(self.scope_stack.is_empty());
    }

    fn insert_definition(&mut self, definiens_node_id: NodeId, name_node: &Rc<TerminalNode>) {
        let definition = Definition {
            name: name_node.unparse(),
            name_node_id: name_node.id(),
            definiens_node_id,
        };
        self.definitions.insert(definiens_node_id, definition);
        if let Some(current_scope) = self.scope_stack.last() {
            current_scope
                .borrow_mut()
                .definitions
                .insert(name_node.unparse(), definiens_node_id);
        } else {
            unreachable!("attempt to insert a definition without a current scope");
        }
    }
}

impl Visitor for Pass {
    fn enter_source_unit(&mut self, node: &SourceUnit) -> bool {
        assert!(self.scope_stack.is_empty());

        self.push_scope(node.node_id);
        true
    }

    fn leave_source_unit(&mut self, _node: &SourceUnit) {
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
        false
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
        false
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
        false
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
        false
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
        false
    }

    fn enter_mapping_key(&mut self, node: &input_ir::MappingKey) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name);
        }
        false
    }

    fn enter_mapping_value(&mut self, node: &input_ir::MappingValue) -> bool {
        if let Some(name) = &node.name {
            self.insert_definition(node.node_id, name);
        }
        false
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
        false
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
        false
    }

    fn enter_constant_definition(&mut self, node: &input_ir::ConstantDefinition) -> bool {
        self.insert_definition(node.node_id, &node.name);
        false
    }

    fn enter_user_defined_value_type_definition(
        &mut self,
        node: &input_ir::UserDefinedValueTypeDefinition,
    ) -> bool {
        self.insert_definition(node.node_id, &node.name);
        false
    }

    fn enter_yul_variable_declaration_statement(
        &mut self,
        node: &input_ir::YulVariableDeclarationStatement,
    ) -> bool {
        for variable in &node.variables {
            self.insert_definition(variable.id(), variable);
        }
        false
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
}
