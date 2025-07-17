use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use solar::ast::Arena;
use solar::config::ImportRemapping;
use solar::interface::source_map::FileName;
use solar::interface::Session;
use solar_sema::thread_local::ThreadLocal;

use crate::sourcify::Contract;

pub fn run(contract: &Contract) -> usize {
    // From Solar's docs: Create a new session with a buffer emitter.
    // This is required to capture the emitted diagnostics and to return them at the end.
    let sess = Session::builder()
        .with_buffer_emitter(solar::interface::ColorChoice::Auto)
        .single_threaded()
        .build();

    let source_map = sess.source_map();

    for map in &contract.import_resolver.source_maps {
        // println!(
        //     "{source_id}: {virtual_path}",
        //     source_id = map.source_id,
        //     virtual_path = map.virtual_path
        // );
        let source = contract
            .read_file(&map.source_id)
            .unwrap_or_else(|_| panic!("Can't read {source_id}", source_id = map.source_id));
        let _ = source_map
            .new_source_file(
                FileName::Real(Path::new(&map.virtual_path).to_path_buf()),
                source,
            )
            .unwrap_or_else(|_| {
                panic!(
                    "Fail to add {source_id} with virtual path {virtual_path}",
                    source_id = map.source_id,
                    virtual_path = map.virtual_path
                )
            });
    }

    let mut pcx = solar_sema::ParsingContext::new(&sess);

    contract
        .import_resolver
        .import_remaps
        .iter()
        .for_each(|remapping| {
            pcx.file_resolver.add_import_remapping(ImportRemapping {
                context: remapping.context.clone().unwrap_or_default(),
                prefix: remapping.prefix.clone(),
                path: remapping.target.clone(),
            });
        });

    let entrypoint = contract
        .import_resolver
        .get_virtual_path(&contract.entrypoint().unwrap())
        .unwrap();

    let parsed_files = sess.enter_parallel(|| {
        let arena = ThreadLocal::<Arena>::new();

        pcx.load_file(Path::new(&entrypoint)).unwrap_or_else(|_| {
            println!(
                "Can't load {entrypoint} for contract {contract}",
                contract = contract.name
            );
        });
        pcx.parse(&arena).sources.len()
    });

    let result = sess
        .emitted_errors()
        .expect("We created sess with a buffer");
    if let Err(err) = result {
        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/solar_parser.log")
            .unwrap();
        writeln!(
            log_file,
            "Fail to parse {contract}: {err}",
            contract = contract.name
        )
        .unwrap();
        0
    } else {
        parsed_files
    }
}
