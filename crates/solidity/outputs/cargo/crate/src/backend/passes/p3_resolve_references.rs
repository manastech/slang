use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use super::p2_collect_definitions::{Definition, Output as Input, Scope};
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::cst::TerminalNode;

pub struct Output {
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub definitions: HashMap<NodeId, Definition>,
    pub references: HashMap<NodeId, Reference>,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let mut pass = Pass::new(input.definitions, input.scopes);

    for (file_id, source_unit) in &files {
        pass.visit_file(file_id, source_unit);
    }
    let definitions = pass.definitions;
    let references = pass.references;

    Output {
        files,
        definitions,
        references,
    }
}

pub struct Reference {
    pub name: String,
    pub node_id: NodeId,
    pub definition_id: Option<NodeId>,
}

struct Pass {
    definitions: HashMap<NodeId, Definition>,
    references: HashMap<NodeId, Reference>,
    scopes: HashMap<NodeId, Rc<RefCell<Scope>>>,
    scope_stack: Vec<NodeId>,
}

impl Pass {
    fn new(
        definitions: HashMap<NodeId, Definition>,
        scopes: HashMap<NodeId, Rc<RefCell<Scope>>>,
    ) -> Self {
        Self {
            definitions,
            references: HashMap::new(),
            scopes,
            scope_stack: Vec::new(),
        }
    }

    fn push_scope(&mut self, node_id: NodeId) {
        if !self.scopes.contains_key(&node_id) {
            unreachable!("attempted to push scope that doesn't exist");
        }
        self.scope_stack.push(node_id);
    }

    fn pop_scope(&mut self) {
        if self.scope_stack.pop().is_none() {
            unreachable!("scope stack empty");
        }
    }

    fn visit_file(&mut self, _file_id: &str, source_unit: &input_ir::SourceUnit) {
        input_ir::visitor::accept_source_unit(source_unit, self);
        assert!(self.scope_stack.is_empty());
    }

    fn insert_reference(&mut self, identifier: &Rc<TerminalNode>) {
        let node_id = identifier.id();
        let name = identifier.unparse();
        let definition_id = self.resolve_name(&name);
        let reference = Reference {
            name,
            node_id,
            definition_id,
        };
        self.references.insert(node_id, reference);
    }

    fn resolve_name(&self, name: &str) -> Option<NodeId> {
        let Some(scope_node_id) = self.scope_stack.last() else {
            unreachable!("attempt to resolve a reference with an empty scope stack");
        };
        let Some(scope) = self.scopes.get(scope_node_id) else {
            unreachable!("scope at the top of the stack doesn't exist");
        };
        scope.borrow().resolve(name)
    }
}

impl input_ir::visitor::Visitor for Pass {
    // Nodes introducing references

    fn leave_import_deconstruction_symbol(&mut self, node: &input_ir::ImportDeconstructionSymbol) {
        self.insert_reference(&node.name);
    }

    fn leave_identifier_path(&mut self, items: &input_ir::IdentifierPath) {
        for item in items {
            self.insert_reference(item);
        }
    }

    fn leave_catch_clause_error(&mut self, node: &input_ir::CatchClauseError) {
        if let Some(name) = &node.name {
            // NOTE: this can either be Panic or Error (ie. built-ins)
            self.insert_reference(name);
        }
    }

    fn enter_expression(&mut self, node: &input_ir::Expression) -> bool {
        match node {
            input_ir::Expression::Identifier(identifier) => {
                self.insert_reference(identifier);
                false
            }
            _ => true,
        }
    }

    fn leave_member_access_expression(&mut self, node: &input_ir::MemberAccessExpression) {
        self.insert_reference(&node.member);
    }

    fn leave_named_argument(&mut self, node: &input_ir::NamedArgument) {
        self.insert_reference(&node.name);
    }

    fn leave_yul_path(&mut self, items: &input_ir::YulPath) {
        for item in items {
            self.insert_reference(item);
        }
    }

    // Scope management

    fn enter_source_unit(&mut self, node: &input_ir::SourceUnit) -> bool {
        assert!(self.scope_stack.is_empty());
        self.push_scope(node.node_id);
        true
    }

    fn leave_source_unit(&mut self, _node: &input_ir::SourceUnit) {
        self.pop_scope();
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        self.push_scope(node.node_id);
        true
    }

    fn leave_contract_definition(&mut self, _node: &input_ir::ContractDefinition) {
        self.pop_scope();
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        self.push_scope(node.node_id);
        true
    }

    fn leave_interface_definition(&mut self, _node: &input_ir::InterfaceDefinition) {
        self.pop_scope();
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        self.push_scope(node.node_id);
        true
    }

    fn leave_library_definition(&mut self, _node: &input_ir::LibraryDefinition) {
        self.pop_scope();
    }

    fn enter_function_definition(&mut self, node: &input_ir::FunctionDefinition) -> bool {
        self.push_scope(node.node_id);
        true
    }

    fn leave_function_definition(&mut self, _node: &input_ir::FunctionDefinition) {
        self.pop_scope();
    }

    fn enter_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) -> bool {
        self.push_scope(node.node_id);
        true
    }

    fn leave_modifier_definition(&mut self, _node: &input_ir::ModifierDefinition) {
        self.pop_scope();
    }

    fn enter_enum_definition(&mut self, _node: &input_ir::EnumDefinition) -> bool {
        // enum definitions cannot contain references
        false
    }

    fn enter_struct_definition(&mut self, node: &input_ir::StructDefinition) -> bool {
        self.push_scope(node.node_id);
        true
    }

    fn leave_struct_definition(&mut self, _node: &input_ir::StructDefinition) {
        self.pop_scope();
    }

    fn enter_event_definition(&mut self, node: &input_ir::EventDefinition) -> bool {
        self.push_scope(node.node_id);
        true
    }

    fn leave_event_definition(&mut self, _node: &input_ir::EventDefinition) {
        self.pop_scope();
    }

    fn enter_error_definition(&mut self, node: &input_ir::ErrorDefinition) -> bool {
        self.push_scope(node.node_id);
        true
    }

    fn leave_error_definition(&mut self, _node: &input_ir::ErrorDefinition) {
        self.pop_scope();
    }

    fn enter_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) -> bool {
        self.push_scope(node.node_id);
        true
    }

    fn leave_yul_function_definition(&mut self, _node: &input_ir::YulFunctionDefinition) {
        self.pop_scope();
    }
}
