use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use anyhow::{bail, Context, Result};
use codegen_schema::{Grammar, ProductionVersions};
use codegen_utils::context::CodegenContext;
use semver::Version;

type ParserTests = BTreeMap<String, BTreeSet<String>>;

pub trait GrammarTestingGeneratorExtensions {
    fn generate_cst_output_tests(
        &self,
        codegen: &mut CodegenContext,
        snapshots_dir: &PathBuf,
        output_dir: &PathBuf,
    ) -> Result<()>;
}

impl GrammarTestingGeneratorExtensions for Grammar {
    fn generate_cst_output_tests(
        &self,
        codegen: &mut CodegenContext,
        data_dir: &PathBuf,
        output_dir: &PathBuf,
    ) -> Result<()> {
        let test_versions = collect_test_versions(self);
        let parser_tests = &collect_parser_tests(codegen, data_dir)?;

        generate_mod_file(
            codegen,
            parser_tests,
            &test_versions,
            &output_dir.join("mod.rs"),
        )?;

        for (parser_name, test_names) in parser_tests {
            generate_unit_test_file(
                codegen,
                parser_name,
                test_names,
                &output_dir.join(format!("{parser_name}.rs")),
            )?;
        }

        return Ok(());
    }
}

fn collect_test_versions<'a>(grammar: &'a Grammar) -> BTreeSet<&'a Version> {
    let mut breaking_versions = BTreeSet::from([
        grammar.manifest.versions.first().unwrap(),
        grammar.manifest.versions.last().unwrap(),
    ]);

    for production in grammar.productions.values().flatten() {
        match &production.versions {
            ProductionVersions::Unversioned(_) => {
                // Nothing to add
            }
            ProductionVersions::Versioned(versions) => {
                for version in versions.keys() {
                    breaking_versions.insert(version);
                }
            }
        }
    }

    return breaking_versions;
}

fn collect_parser_tests(codegen: &mut CodegenContext, data_dir: &PathBuf) -> Result<ParserTests> {
    let input_files = codegen.collect_files_recursively(data_dir)?;
    let mut parser_tests: ParserTests = ParserTests::new();

    for file_path in input_files {
        if codegen.get_generated_dir(&file_path).is_some() {
            continue; // skip generated files
        }

        let parts = file_path
            .strip_prefix(data_dir)?
            .iter()
            .map(|p| Ok(p.to_str().context(format!("Failed to parse part: {p:?}"))?))
            .collect::<Result<Vec<&str>>>()?;

        match &parts[..] {
            [parser_name, test_name, "input.sol"] => {
                let parser_tests = parser_tests
                    .entry(parser_name.to_string())
                    .or_insert_with(|| BTreeSet::new());

                parser_tests.insert(test_name.to_string());
            }
            _ => {
                bail!("Invalid test input. Should be in the form of '<tests-dir>/PARSER_NAME/TEST_NAME/input.sol', but found: {file_path:?}");
            }
        };
    }

    return Ok(parser_tests);
}

fn generate_mod_file(
    codegen: &mut CodegenContext,
    parser_tests: &ParserTests,
    test_versions: &BTreeSet<&Version>,
    mod_file_path: &PathBuf,
) -> Result<()> {
    let module_declarations = parser_tests
        .keys()
        .map(|parser_name| format!("mod {parser_name};"))
        .collect::<String>();

    let test_versions_size = test_versions.len();

    let test_versions = test_versions
        .iter()
        .map(|version| format!("\"{version}\","))
        .collect::<String>();

    let parser_name_match_arms = parser_tests
        .keys()
        .map(|parser_name| format!("\"{parser_name}\" => parsers.{parser_name},"))
        .collect::<String>();

    let contents = format!(
        "
            {module_declarations}

            use std::rc::Rc;

            use anyhow::{{bail, Result}};
            use solidity_rust_lib::generated::{{
                cst,
                parse::{{ParserType, Parsers}},
            }};

            pub const TEST_VERSIONS: [&str; {test_versions_size}] = [
                {test_versions}
            ];

            pub fn get_parser<'a>(
                parsers: Parsers<'a>,
                parser_name: &str,
            ) -> Result<ParserType<'a, Rc<cst::Node>>> {{
                return Ok(match parser_name {{
                    {parser_name_match_arms}
                    _ => bail!(\"Unrecognized parser_name: {{parser_name}}\"),
                }});
            }}
        "
    );

    return codegen.write_file(mod_file_path, &contents);
}

fn generate_unit_test_file(
    codegen: &mut CodegenContext,
    parser_name: &str,
    test_names: &BTreeSet<String>,
    unit_test_file_path: &PathBuf,
) -> Result<()> {
    let unit_tests = test_names
        .iter()
        .map(|test_name| {
            format!(
                "
                    #[test]
                    fn {test_name}() -> Result<()> {{
                        return run(\"{parser_name}\", \"{test_name}\");
                    }}
                "
            )
        })
        .collect::<String>();

    let contents = format!(
        "
            use crate::cst_output::run;
            use anyhow::Result;

            {unit_tests}
        "
    );

    return codegen.write_file(unit_test_file_path, &contents);
}