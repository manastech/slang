use std::collections::HashMap;
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use super::p2_collect_definitions::Output as Input;
use crate::backend::binder::Binder;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::cst::TerminalNode;

pub struct Output {
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let mut pass = Pass::new(input.binder);

    for (file_id, source_unit) in &files {
        pass.visit_file(file_id, source_unit);
    }
    let binder = pass.binder;

    Output { files, binder }
}

struct Pass {
    binder: Binder,
}

impl Pass {
    fn new(binder: Binder) -> Self {
        Self { binder }
    }

    fn visit_file(&mut self, _file_id: &str, source_unit: &input_ir::SourceUnit) {
        input_ir::visitor::accept_source_unit(source_unit, self);
    }

    fn insert_reference_scoped_at(&mut self, scope_node_id: NodeId, identifier: &Rc<TerminalNode>) {
        self.binder.insert_reference_scoped_at(
            scope_node_id,
            identifier.id(),
            identifier.unparse(),
        );
    }

    fn insert_reference(&mut self, identifier: &Rc<TerminalNode>) {
        self.insert_reference_scoped_at(identifier.id(), identifier);
    }

    // TODO: temporary until we can resolve scope of expressions and other definitions
    fn insert_unresolved_reference(&mut self, identifier: &Rc<TerminalNode>) {
        self.binder
            .insert_unresolved_reference(identifier.id(), identifier.unparse());
    }
}

impl input_ir::visitor::Visitor for Pass {
    fn leave_import_deconstruction_symbol(&mut self, node: &input_ir::ImportDeconstructionSymbol) {
        self.insert_reference_scoped_at(node.node_id, &node.name);
    }

    fn leave_identifier_path(&mut self, items: &input_ir::IdentifierPath) {
        // TODO: we can only attempt to resolve the first symbol in the path, as
        // the other will need to resolve in the scope of the previous one
        if let Some(first_item) = items.first() {
            self.insert_reference(first_item);
            for item in items.iter().skip(1) {
                self.insert_unresolved_reference(item);
            }
        }
    }

    fn leave_catch_clause_error(&mut self, node: &input_ir::CatchClauseError) {
        if let Some(name) = &node.name {
            // NOTE: this can either be Panic or Error (ie. built-ins)
            self.insert_reference_scoped_at(node.node_id, name);
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
        self.insert_unresolved_reference(&node.member);
    }

    fn leave_named_argument(&mut self, node: &input_ir::NamedArgument) {
        self.insert_reference_scoped_at(node.node_id, &node.name);
    }

    fn leave_yul_path(&mut self, items: &input_ir::YulPath) {
        // TODO: we can only attempt to resolve the first symbol in the path, as
        // the other will need to resolve in the scope of the previous one
        if let Some(first_item) = items.first() {
            self.insert_reference(first_item);
            for item in items.iter().skip(1) {
                self.insert_unresolved_reference(item);
            }
        }
    }
}
