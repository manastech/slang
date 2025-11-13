#![allow(missing_docs)]

use std::collections::HashMap;

use binder::Binder;
use types::TypeRegistry;

use self::abi::ContractAbi;
use crate::backend::ir::ir2_flat_contracts as output_ir;
use crate::compilation::CompilationUnit;

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
