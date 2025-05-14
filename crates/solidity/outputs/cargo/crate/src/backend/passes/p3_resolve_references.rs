use std::collections::HashMap;
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use super::p2_collect_definitions::{Definition, Output as Input};
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::cst::TerminalNode;

pub struct Output {
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub definitions: HashMap<NodeId, Definition>,
    pub references: HashMap<NodeId, Reference>,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let mut pass = Pass::new(input.definitions);

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
}

impl Pass {
    fn new(definitions: HashMap<NodeId, Definition>) -> Self {
        Self {
            definitions,
            references: HashMap::new(),
        }
    }

    fn visit_file(&mut self, _file_id: &str, source_unit: &input_ir::SourceUnit) {
        input_ir::visitor::accept_source_unit(source_unit, self);
    }

    fn insert_reference(&mut self, identifier: &Rc<TerminalNode>) {
        let node_id = identifier.id();
        let name = identifier.unparse();
        let reference = Reference {
            name,
            node_id,
            definition_id: None,
        };
        self.references.insert(node_id, reference);
    }
}

impl input_ir::visitor::Visitor for Pass {
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
}
