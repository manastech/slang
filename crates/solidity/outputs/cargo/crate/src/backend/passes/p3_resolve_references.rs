use std::collections::HashMap;

use metaslang_cst::nodes::NodeId;

use super::p2_collect_definitions::{Definition, Output as Input};
use crate::backend::l2_flat_contracts::{self as input_ir};

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

    fn visit_file(&mut self, _file_id: &str, _source_unit: &input_ir::SourceUnit) {
        todo!()
    }
}

impl input_ir::visitor::Visitor for Pass {}
