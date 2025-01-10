use std::path::Path;
use std::rc::Rc;

use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity::bindings::BindingGraph;
use slang_solidity::compilation::{AddFileResponse, InternalCompilationBuilder};
use slang_solidity::cst::{Cursor, TerminalKind};

pub const SOLC_VERSION: Version = Version::new(0, 7, 1);

const SOURCE: &str = "pools/weighted/WeightedPool.sol";

const BASE_PATH: &str = "crates/solidity/outputs/npm/tests/src/compilation/inputs/0x01abc00E86C7e258823b9a055Fd62cA6CF61a163/sources/contracts";

fn resolve_path(context_path: &str, path_to_resolve: &Cursor) -> String {
    let path = path_to_resolve.node().unparse();
    let path = path
        .strip_prefix(|c| matches!(c, '"' | '\''))
        .unwrap()
        .strip_suffix(|c| matches!(c, '"' | '\''))
        .unwrap();

    let context_path = Path::new(context_path).parent().expect("Wrong path");
    let path = context_path.join(path).canonicalize().expect("Wrong path");
    let path_str = path.as_os_str().to_str().expect("Wrong path");

    if path.exists() {
        String::from(path_str)
    } else {
        panic!("Path doesn't exist: {path_str}")
    }
}

fn add_file(seen_files: &mut Vec<String>, builder: &mut InternalCompilationBuilder, file: &str) {
    let file_s = String::from(file);
    if !seen_files.contains(&file_s) {
        println!("Processing {file}");
        seen_files.push(file_s.clone());
        let contents = Path::new(file).read_to_string().expect("Can't read file");
        let AddFileResponse { import_paths } = builder.add_file(file_s, contents.as_str());
        for import_path in import_paths {
            let file_id = resolve_path(file, &import_path);
            builder
                .resolve_import(file, &import_path, file_id.clone())
                .expect("Cant resolve import");
            add_file(seen_files, builder, file_id.as_str());
        }
    }
}

pub fn setup() -> (Cursor, Rc<BindingGraph>) {
    let mut builder =
        InternalCompilationBuilder::create(SOLC_VERSION).expect("Can't create CompilationBuilder");

    let path = Path::repo_path(BASE_PATH).join(SOURCE);
    let path = path.canonicalize().expect("Wrong path");
    let path_str = path.as_os_str().to_str().expect("Wrong path");

    add_file(&mut vec![], &mut builder, path_str);

    let unit = builder.build();
    if let Ok(graph) = unit.binding_graph() {
        (
            unit.file(path_str)
                .expect("Can't get file")
                .create_tree_cursor(),
            Rc::clone(graph),
        )
    } else {
        panic!("Can't get binding graph")
    }
}

pub fn run(data: (Cursor, Rc<BindingGraph>)) {
    let (mut cursor, binding_graph) = data;

    let mut ambiguous_refs = 0;
    let mut refs = 0;
    let mut defs = 0;
    let mut empty_refs = 0;
    let mut neither_def_nor_ref = 0;

    while cursor.go_to_next_terminal_with_kind(TerminalKind::Identifier) {
        let definition = binding_graph.definition_at(&cursor);
        let reference = binding_graph.reference_at(&cursor);

        if let Some(real_reference) = &reference {
            let defs = real_reference.definitions().len();
            if defs > 1 {
                ambiguous_refs += 1;
            } else if defs > 0 {
                refs += 1;
            } else {
                empty_refs += 1;
            }
        }

        if definition.is_some() {
            defs += 1;
        }

        if definition.is_none() && reference.is_none() {
            neither_def_nor_ref += 1;
        }
    }

    assert_eq!(462, refs);
    assert_eq!(143, defs);
    assert_eq!(1, neither_def_nor_ref);
    assert_eq!(ambiguous_refs, 10);
    assert_eq!(empty_refs, 0);
}
