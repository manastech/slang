use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;
use slang_solidity::backend::build_context;
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig};
use slang_solidity::utils::LanguageFacts;

#[test]
fn test_get_contracts() -> Result<()> {
    use slang_solidity::backend::abi::FunctionAbiType as FType;

    let mut compilation_builder =
        CompilationBuilder::create(LanguageFacts::LATEST_VERSION, Config::create())?;
    compilation_builder.add_file("main.sol")?;
    let compilation_unit = Rc::new(compilation_builder.build());
    let backend_context = build_context(compilation_unit);
    let contracts = backend_context.contracts();
    assert_eq!(1, contracts.len());
    assert_eq!("Counter", contracts[0].name);
    assert_eq!("main.sol", contracts[0].file_id);
    assert_eq!(6, contracts[0].functions.len());

    let functions = contracts[0]
        .functions
        .iter()
        .map(|function| (function.kind, function.name.clone()))
        .collect::<Vec<_>>();
    let expected_functions = &[
        (FType::Constructor, None),
        (FType::Function, Some("count".to_string())),
        (FType::Function, Some("increment".to_string())),
        (FType::Function, Some("enable".to_string())),
        (FType::Function, Some("disable".to_string())),
        (FType::Function, Some("click".to_string())),
    ];
    assert_eq!(expected_functions[..], functions[..]);

    Ok(())
}

const FILES: &[(&str, &str)] = &[
    (
        "main.sol",
        r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

import {Ownable} from "ownable.sol";

contract Counter is Ownable {
    enum State { DISABLED, ENABLED }

    State _state;
    uint _count;
    mapping (address => uint) _clickers;

    constructor(uint initial) Ownable() {
        _count = initial;
        _state = State.DISABLED;
    }
    function count() public view returns (uint) {
        return _count;
    }
    function increment(uint delta) public onlyOwner returns (uint) {
        _count += delta;
        return _count;
    }
    function enable() public onlyOwner {
        _state = State.ENABLED;
    }
    function disable() public onlyOwner {
        _state = State.DISABLED;
    }
    function click() public returns (uint) {
        require(_state == State.ENABLED, "counter is disabled");
        _count += 1;
        _clickers[msg.sender] += 1;
        return _clickers[msg.sender];
    }
}
"#,
    ),
    (
        "ownable.sol",
        r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

abstract contract Ownable {
    address _owner;
    constructor() {
        _owner = msg.sender;
    }
    modifier onlyOwner() {
        require(msg.sender == _owner, "Only owner allowed");
        _;
    }
}
"#,
    ),
];

struct Config {
    files: HashMap<String, String>,
}

impl Config {
    fn create() -> Self {
        let files = FILES
            .iter()
            .map(|(file_id, contents)| ((*file_id).to_string(), (*contents).to_string()))
            .collect();
        Self { files }
    }
}

impl CompilationBuilderConfig for Config {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> Result<Option<String>> {
        Ok(self.files.get(file_id).map(|contents| contents.to_owned()))
    }

    fn resolve_import(
        &mut self,
        _source_file_id: &str,
        import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> Result<Option<String>> {
        let target_path = import_path_cursor.node().unparse();
        let target_file_id = &target_path[1..target_path.len() - 1];

        if self.files.contains_key(target_file_id) {
            Ok(Some(target_file_id.to_owned()))
        } else {
            Ok(None)
        }
    }
}
