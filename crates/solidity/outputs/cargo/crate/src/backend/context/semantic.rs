use std::collections::HashMap;
use std::rc::Rc;

use semver::Version;

use crate::backend::binder::Binder;
pub use crate::backend::ir::ir2_flat_contracts as output_ir;
use crate::backend::passes;
use crate::backend::types::{Type, TypeId, TypeRegistry};
use crate::compilation::CompilationUnit;
use crate::cst::NodeId;

pub struct SemanticAnalysis {
    pub(crate) compilation_unit: Rc<CompilationUnit>,
    pub(crate) files: HashMap<String, output_ir::SourceUnit>,
    pub(crate) binder: Binder,
    pub(crate) types: TypeRegistry,
}

impl SemanticAnalysis {
    pub(crate) fn create(compilation_unit: &Rc<CompilationUnit>) -> Self {
        let mut semantic_analysis = Self::new(compilation_unit);

        passes::p2_collect_definitions::run(&mut semantic_analysis);
        passes::p3_linearise_contracts::run(&mut semantic_analysis);
        passes::p4_type_definitions::run(&mut semantic_analysis);
        passes::p5_resolve_references::run(&mut semantic_analysis);

        semantic_analysis
    }

    fn new(compilation_unit: &Rc<CompilationUnit>) -> Self {
        let language_version = compilation_unit.language_version().clone();
        let binder = Binder::new();
        let types = TypeRegistry::new(language_version);

        let files = passes::p1_flatten_contracts::run(passes::p0_build_ast::run(Rc::clone(
            compilation_unit,
        )))
        .files;

        Self {
            compilation_unit: Rc::clone(compilation_unit),
            files,
            binder,
            types,
        }
    }
}

impl SemanticAnalysis {
    pub fn language_version(&self) -> &Version {
        self.compilation_unit.language_version()
    }

    pub fn definition_canonical_name(&self, definition_id: NodeId) -> String {
        self.binder
            .find_definition_by_id(definition_id)
            .unwrap()
            .identifier()
            .unparse()
    }

    pub fn type_canonical_name(&self, type_id: TypeId) -> String {
        match self.types.get_type_by_id(type_id) {
            Type::Address { .. } => "address".to_string(),
            Type::Array { element_type, .. } => {
                format!(
                    "{element}[]",
                    element = self.type_canonical_name(*element_type)
                )
            }
            Type::Boolean => "bool".to_string(),
            Type::ByteArray { width } => format!("bytes{width}"),
            Type::Bytes { .. } => "bytes".to_string(),
            Type::FixedPointNumber {
                signed,
                bits,
                precision_bits,
            } => format!(
                "{prefix}{bits}x{precision_bits}",
                prefix = if *signed { "fixed" } else { "ufixed" },
            ),
            Type::Function(_) => "function".to_string(),
            Type::Integer { signed, bits } => format!(
                "{prefix}{bits}",
                prefix = if *signed { "int" } else { "uint" }
            ),
            Type::Literal(_) => "literal".to_string(),
            Type::Mapping {
                key_type_id,
                value_type_id,
            } => format!(
                "mapping({key_type} => {value_type})",
                key_type = self.type_canonical_name(*key_type_id),
                value_type = self.type_canonical_name(*value_type_id)
            ),
            Type::String { .. } => "string".to_string(),
            Type::Tuple { types } => format!(
                "({types})",
                types = types
                    .iter()
                    .map(|type_id| self.type_canonical_name(*type_id))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Type::Contract { definition_id }
            | Type::Enum { definition_id }
            | Type::Interface { definition_id }
            | Type::Struct { definition_id, .. }
            | Type::UserDefinedValue { definition_id } => {
                self.definition_canonical_name(*definition_id)
            }
            Type::Void => "void".to_string(),
        }
    }
}
