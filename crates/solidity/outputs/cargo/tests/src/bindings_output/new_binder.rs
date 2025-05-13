use std::rc::Rc;

use anyhow::{anyhow, Result};
use metaslang_bindings::PathResolver;
use semver::Version;
use slang_solidity::{
    backend::passes, bindings::BindingLocation, compilation::InternalCompilationBuilder,
};

use crate::resolver::TestsPathResolver;

use super::runner::ParsedPart;

pub(crate) fn test_new_binder(
    group_name: &str,
    test_name: &str,
    version: &Version,
    parsed_parts: &[ParsedPart<'_>],
) -> Result<()> {
    if *version != Version::parse("0.8.29")? {
        // we only support a single version for now
        return Ok(());
    }
    let test_id = format!("{group_name}/{test_name}");
    if parsed_parts
        .iter()
        .any(|parsed_part| !parsed_part.parse_output.is_valid())
    {
        println!("[{test_id}] Skipping due to invalid parse output");
        return Ok(());
    }

    let mut internal_builder = InternalCompilationBuilder::create(version.clone())?;
    let import_resolver = TestsPathResolver {};

    for parsed_part in parsed_parts {
        let id = parsed_part.path;
        let contents = parsed_part.contents;
        let add_file_response = internal_builder.add_file(id.into(), contents);
        for import_path in &add_file_response.import_paths {
            if let Some(destination_file_id) = import_resolver.resolve_path(id, import_path) {
                internal_builder.resolve_import(id, import_path, destination_file_id)?;
            } else {
                unreachable!(
                    "cannot resolve {import_path} in the context of {id}",
                    import_path = import_path.node().unparse()
                );
            }
        }
    }
    let compilation_unit = internal_builder.build();
    let binding_graph = Rc::clone(compilation_unit.binding_graph());

    let data = passes::p0_build_ast::run(&compilation_unit).map_err(|s| anyhow!(s))?;
    let data = passes::p1_flatten_contracts::run(&data);
    let data = passes::p2_collect_definitions::run(&data);

    let mut user_definitions = 0;
    for definition in binding_graph.all_definitions() {
        if matches!(definition.definiens_location(), BindingLocation::BuiltIn(_)) {
            continue;
        }
        let definition_id = definition.id();

        if let Some(new_definition) = data.definitions.get(&definition_id) {
            if new_definition.name_node_id != definition.get_cursor().node().id() {
                println!("[{test_id}] Definition {definition:?} differs in name node ID");
            }
        } else {
            println!("[{test_id}] Definition {definition:?} not found with new binder");
        }
        user_definitions += 1;
    }
    if user_definitions != data.definitions.len() {
        println!("[{test_id}] Number of definitions mismatch: binding graph = {user_definitions}, new_binder = {new_len}", new_len = data.definitions.len());
    }

    Ok(())
}
