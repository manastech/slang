use slang_solidity::backend::binder::Resolution;
use slang_solidity::backend::passes;
use slang_solidity::backend::passes::p5_resolve_references::Output;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{NodeKind, TerminalKindExtensions};

use crate::compilation_builder::create_compilation_unit;

pub fn setup(project: &str) -> CompilationUnit {
    create_compilation_unit(super::setup::setup(project)).unwrap()
}

fn build(unit: CompilationUnit) -> Output {
    let data = passes::p0_build_ast::run(unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_linearise_contracts::run(data);
    let data = passes::p4_type_definitions::run(data);
    std::hint::black_box(passes::p5_resolve_references::run(data))
}

pub fn setup_for_cleanup(project: &str) -> Output {
    build(create_compilation_unit(super::setup::setup(project)).unwrap())
}

pub fn run(unit: CompilationUnit) -> Output {
    let data = build(unit);
    let mut ids = 0;

    for file in data.compilation_unit.files() {
        let mut cursor = file.create_tree_cursor();

        while cursor.go_to_next_terminal() {
            if !matches!(cursor.node().kind(), NodeKind::Terminal(kind) if kind.is_identifier()) {
                continue;
            }

            ids += 1;
            let node_id = cursor.node().id();
            if data
                .binder
                .find_definition_by_identifier_node_id(node_id)
                .is_none()
                && data
                    .binder
                    .find_reference_by_identifier_node_id(node_id)
                    .is_none_or(|reference| reference.resolution == Resolution::Unresolved)
            {
                panic!(
                    "Unbound identifier: '{value}' in '{file_path}:{line}:{column}'.",
                    value = cursor.node().unparse(),
                    file_path = file.id(),
                    line = cursor.text_range().start.line + 1,
                    column = cursor.text_range().start.column + 1,
                );
            }
        }
    }
    assert_ne!(ids, 0);

    data
}
