use std::path::PathBuf;

use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;
use quote::{format_ident, quote};

use super::{boilerplate, code_generator::CodeGenerator, naming, rust_lib_boilerplate};

impl CodeGenerator {
    pub fn write_rust_lib_sources(
        &self,
        grammar: &Grammar,
        codegen: &mut CodegenContext,
        output_dir: &PathBuf,
    ) {
        codegen
            .write_file(
                &output_dir.join("mod.rs"),
                &rust_lib_boilerplate::mod_head().to_string(),
            )
            .unwrap();

        codegen
            .write_file(
                &output_dir.join("lex.rs"),
                &boilerplate::lex_head().to_string(),
            )
            .unwrap();

        codegen
            .write_file(
                &output_dir.join("cst.rs"),
                &format!(
                    "{}\n{}",
                    boilerplate::cst_head(),
                    rust_lib_boilerplate::cst_visitor_head()
                ),
            )
            .unwrap();

        self.write_parser_source(grammar, codegen, output_dir);

        codegen
            .write_file(
                &output_dir.join("language.rs"),
                &format!(
                    "{}

                    impl Language {{
                        {}
                    }}
                    ",
                    rust_lib_boilerplate::language_head(),
                    self
                        .parsers
                        .iter()
                        .map(|(name, parser)| {
                            let field_name = naming::to_field_name_ident(&name);
                            let method_name = format_ident!("parse_{}", field_name);
                            format!(
                                "{}\n{}",
                                parser
                                    .comment
                                    .iter()
                                    .map(|s| format!("// {}", s))
                                    .collect::<Vec<_>>()
                                    .join("\n"),
                                quote!(
                                    pub fn #method_name(&self, source: &str) -> Rc<cst::Node> {
                                        let (node, _errs) = self.parsers.#field_name.parse_recovery(source);
                                        node.unwrap()
                                    }
                                )
                            )
                        })
                        .collect::<Vec<_>>().join("\n\n"),
                ),
            )
            .unwrap();

        // Do the kinds last, because the code generation steps above may have added new kinds
        codegen
            .write_file(
                &output_dir.join("kinds.rs"),
                &format!(
                    "{}

                    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
                    pub enum Token {{
                        {}
                    }}

                    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
                    pub enum Rule {{
                        {}
                    }}
                    ",
                    boilerplate::kinds_head(),
                    self.token_kinds
                        .keys()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                    self.rule_kinds
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            )
            .unwrap();
    }
}