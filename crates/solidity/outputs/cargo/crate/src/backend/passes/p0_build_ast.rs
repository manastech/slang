use std::collections::HashMap;
use std::rc::Rc;

use crate::backend::ir::ir1_structured_ast::{builder, SourceUnit};
use crate::compilation::CompilationUnit;

pub struct Output {
    pub compilation_unit: Rc<CompilationUnit>,
    pub files: HashMap<String, SourceUnit>,
}

pub fn run(input: Rc<CompilationUnit>) -> Output {
    let mut files = HashMap::new();
    for file in &input.files() {
        if let Some(source_unit) = builder::build_source_unit(file.tree()) {
            files.insert(file.id().into(), source_unit);
        }
    }
    Output {
        compilation_unit: input,
        files,
    }
}
