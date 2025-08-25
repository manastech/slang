#![allow(clippy::exit)]

use std::hint::black_box;
use std::rc::Rc;

use iai_callgrind::{library_benchmark, library_benchmark_group, main, LibraryBenchmarkConfig};
use slang_solidity::backend::passes::p5_resolve_references::Output;
use slang_solidity::bindings::BindingGraph;
use slang_solidity::compilation::CompilationUnit;
use solidity_testing_perf_cargo::dataset::SolidityProject;
use solidity_testing_perf_cargo::tests::bindings_resolve::BuiltBindingGraph;
use solidity_testing_perf_cargo::tests::{self};

mod __dependencies_used_in_lib__ {
    use {
        anyhow as _, infra_utils as _, paste as _, semver as _, serde as _, serde_json as _,
        slang_solidity as _, solar as _, solidity_testing_perf_utils as _, tree_sitter as _,
        tree_sitter_solidity as _,
    };
}

const WEIGHTED_POOL: &str = "circular_imports_weighted_pool";
const MERKLE_PROOF: &str = "three_quarters_file_merkle_proof";

#[library_benchmark(setup = tests::parser::setup)]
#[bench::fast(MERKLE_PROOF)]
#[bench::slow(WEIGHTED_POOL)]
fn bench_parser(project: &'static SolidityProject) -> Rc<CompilationUnit> {
    black_box(tests::parser::run(project))
}

#[library_benchmark(setup = tests::bindings_build::setup)]
#[bench::fast(MERKLE_PROOF)]
#[bench::slow(WEIGHTED_POOL)]
fn bench_parser_cleanup(unit: Rc<CompilationUnit>) {
    black_box(unit);
}

library_benchmark_group!(
    name = bench_parser_group;
    benchmarks = bench_parser, bench_parser_cleanup,
);

#[library_benchmark(setup = tests::bindings_build::setup)]
#[bench::fast(MERKLE_PROOF)]
#[bench::slow(WEIGHTED_POOL)]
fn bench_graph_binder_build(unit: Rc<CompilationUnit>) -> (Rc<CompilationUnit>, Rc<BindingGraph>) {
    let graph = black_box(tests::bindings_build::run(Rc::clone(&unit)));
    (unit, graph)
}

#[library_benchmark(setup = tests::bindings_resolve::setup)]
#[bench::fast(MERKLE_PROOF)]
#[bench::slow(WEIGHTED_POOL)]
fn bench_graph_binder_resolve(deps: BuiltBindingGraph) -> BuiltBindingGraph {
    black_box(tests::bindings_resolve::run(deps))
}

#[library_benchmark(setup = tests::bindings_resolve::setup)]
#[bench::fast(MERKLE_PROOF)]
#[bench::slow(WEIGHTED_POOL)]
fn bench_graph_binder_cleanup(deps: BuiltBindingGraph) {
    black_box(deps);
}

library_benchmark_group!(
    name = bench_graph_binder_group;
    benchmarks = bench_graph_binder_build, bench_graph_binder_resolve, bench_graph_binder_cleanup,
);

#[library_benchmark(setup = tests::binder::setup)]
#[bench::fast(MERKLE_PROOF)]
#[bench::slow(WEIGHTED_POOL)]
fn bench_binder_binder(unit: CompilationUnit) -> Output {
    black_box(tests::binder::run(unit))
}

#[library_benchmark(setup = tests::binder::setup_for_cleanup)]
#[bench::fast(MERKLE_PROOF)]
#[bench::slow(WEIGHTED_POOL)]
fn bench_binder_cleanup(output: Output) {
    black_box(output);
}

library_benchmark_group!(
    name = bench_binder_group;
    benchmarks = bench_binder_binder, bench_binder_cleanup,
);

main!(
    config = LibraryBenchmarkConfig::default().env_clear(false);

    library_benchmark_groups = bench_parser_group, bench_graph_binder_group, bench_binder_group,
);
