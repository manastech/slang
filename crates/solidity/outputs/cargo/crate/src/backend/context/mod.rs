use std::collections::HashMap;
use std::rc::Rc;

pub use semantic::{output_ir, SemanticAnalysis};

use super::abi::ContractAbi;
use super::binder::Binder;
use super::passes;
use super::types::TypeRegistry;
use crate::compilation::CompilationUnit;

pub mod semantic;

pub struct BackendContext {
    pub(crate) compilation_unit: Rc<CompilationUnit>,
    pub(crate) semantic_analysis: SemanticAnalysis,
    pub(crate) contracts: Vec<ContractAbi>,
}

impl BackendContext {
    pub(crate) fn build(compilation_unit: Rc<CompilationUnit>) -> Self {
        let semantic_analysis = SemanticAnalysis::create(&compilation_unit);
        let contracts = passes::p6_compute_abi::run(&semantic_analysis);

        Self {
            compilation_unit,
            semantic_analysis,
            contracts,
        }
    }

    pub fn compilation_unit(&self) -> &CompilationUnit {
        &self.compilation_unit
    }

    pub fn files(&self) -> &HashMap<String, output_ir::SourceUnit> {
        &self.semantic_analysis.files
    }

    pub fn get_file_ir(&self, file_id: &str) -> Option<&output_ir::SourceUnit> {
        self.semantic_analysis.files.get(file_id)
    }

    pub fn binder(&self) -> &Binder {
        &self.semantic_analysis.binder
    }

    pub fn types(&self) -> &TypeRegistry {
        &self.semantic_analysis.types
    }

    pub fn contracts(&self) -> &[ContractAbi] {
        &self.contracts
    }
}
