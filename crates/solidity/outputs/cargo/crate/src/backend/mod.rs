#![allow(missing_docs)]

use crate::compilation::CompilationUnit;

pub mod abi;
pub mod binder;
pub mod built_ins;
pub mod context;
pub mod ir;
pub mod passes;
pub mod types;

pub use context::BackendContext;

pub fn build_context(compilation_unit: CompilationUnit) -> BackendContext {
    BackendContext::build(compilation_unit)
}
