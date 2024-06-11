// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "generated/binding_rules.rs"]
mod binding_rules;

pub use metaslang_graph_builder::functions::Functions;
use metaslang_graph_builder::{ast, ParseError};
pub use metaslang_graph_builder::{ExecutionConfig, ExecutionError, NoCancellation, Variables};

use crate::cst::KindTypes;

pub type File = ast::File<KindTypes>;
pub type Graph = metaslang_graph_builder::graph::Graph<KindTypes>;

pub fn get_stack_graph_builder() -> Result<File, ParseError> {
    File::from_str(binding_rules::BINDING_RULES_SOURCE)
}
