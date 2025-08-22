use std::{hint::black_box, rc::Rc};

use iai_callgrind::{library_benchmark, library_benchmark_group, main, LibraryBenchmarkConfig};
use slang_solidity::{bindings::BindingGraph, compilation::CompilationUnit};
use solidity_testing_perf_cargo::{
    dataset::SolidityProject,
    tests::{self, bindings_resolve::BuiltBindingGraph},
};

mod __dependencies_used_in_lib__ {
    use {
        anyhow as _, infra_utils as _, paste as _, semver as _, serde as _, serde_json as _,
        slang_solidity as _, solar as _, solidity_testing_perf_utils as _, tree_sitter as _,
        tree_sitter_solidity as _,
    };
}

static WEIGHTED_POOL: &'static str = "circular_imports_weighted_pool";
static MERKLE_PROOF: &'static str = "three_quarters_file_merkle_proof";

#[library_benchmark(setup = tests::parser::setup)]
#[bench::short(MERKLE_PROOF)]
#[bench::long(WEIGHTED_POOL)]
fn bench_binder_parser(project: &'static SolidityProject) -> Rc<CompilationUnit> {
    black_box(tests::parser::run(project))
}

#[library_benchmark(setup = tests::bindings_build::setup)]
#[bench::short(MERKLE_PROOF)]
#[bench::long(WEIGHTED_POOL)]
fn bench_binder_bindings_build(unit: Rc<CompilationUnit>) -> Rc<BindingGraph> {
    black_box(tests::bindings_build::run(unit))
}

#[library_benchmark(setup = tests::bindings_resolve::setup)]
#[bench::short(MERKLE_PROOF)]
#[bench::long(WEIGHTED_POOL)]
fn bench_binder_bindings_resolve(deps: BuiltBindingGraph) {
    black_box(tests::bindings_resolve::run(deps))
}

#[library_benchmark(setup = tests::binder::setup)]
#[bench::short(MERKLE_PROOF)]
#[bench::long(WEIGHTED_POOL)]
fn bench_binder_binder(unit: CompilationUnit) -> CompilationUnit {
    black_box(tests::binder::run(unit))
}

library_benchmark_group!(
    name = bench_binder_group;
    benchmarks = bench_binder_parser, bench_binder_bindings_build, bench_binder_bindings_resolve, bench_binder_binder,
);

main!(
    config = LibraryBenchmarkConfig::default().env_clear(false);

    library_benchmark_groups = bench_binder_group,
);
