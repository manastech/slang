#![allow(missing_docs)]

use std::collections::HashMap;

use abi::ContractAbi;
use binder::Binder;
use ir::ir2_flat_contracts as output_ir;
use types::{Type, TypeId, TypeRegistry};

use crate::compilation::CompilationUnit;
use crate::cst::NodeId;

pub mod abi;
pub mod binder;
pub mod built_ins;
pub mod ir;
pub mod passes;
pub mod types;

pub type BinderOutput = passes::p5_resolve_references::Output;

pub fn build_binder_output(compilation_unit: CompilationUnit) -> BinderOutput {
    let data = passes::p0_build_ast::run(compilation_unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_linearise_contracts::run(data);
    let data = passes::p4_type_definitions::run(data);
    passes::p5_resolve_references::run(data)
}

pub struct BackendContext {
    compilation_unit: CompilationUnit,
    files: HashMap<String, output_ir::SourceUnit>,
    binder: Binder,
    types: TypeRegistry,
    contracts: Vec<ContractAbi>,
}

impl BackendContext {
    pub fn build(compilation_unit: CompilationUnit) -> Self {
        let output = build_binder_output(compilation_unit);
        let contracts = passes::p6_compute_abi::run(&output);
        Self {
            compilation_unit: output.compilation_unit,
            files: output.files,
            binder: output.binder,
            types: output.types,
            contracts,
        }
    }

    pub fn compilation_unit(&self) -> &CompilationUnit {
        &self.compilation_unit
    }

    pub fn get_file_ir(&self, file_id: &str) -> Option<&output_ir::SourceUnit> {
        self.files.get(file_id)
    }

    pub fn binder(&self) -> &Binder {
        &self.binder
    }

    pub fn types(&self) -> &TypeRegistry {
        &self.types
    }

    pub fn contracts(&self) -> &[ContractAbi] {
        &self.contracts
    }
}

impl BackendContext {
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
