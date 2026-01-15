use std::rc::Rc;

use anyhow::Result;
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};
use slang_solidity::utils::LanguageFacts;

pub const MAIN_ID: &str = "main.sol";
pub const MAIN_SOL_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

abstract contract Ownable {
  address _owner;
  constructor() {
    _owner = msg.sender;
  }
  modifier onlyOwner() {
    require(_owner == msg.sender);
    _;
  }
  function checkOwner(address addr) internal returns (bool) {
    return _owner == addr;
  }
}

contract Counter is Ownable {
  uint _count;
  uint _unused;
  constructor(uint initialCount) {
    _count = initialCount;
  }
  function count() public view returns (uint) {
    return _count;
  }
  function increment(uint delta, uint multiplier) public onlyOwner returns (uint) {
    require(delta > 0, "Delta must be positive");
    _count += delta;
    return _count;
  }
  function unusedDecrement() private {
    require(checkOwner(msg.sender));
    _count -= 1;
  }
}
"#;

pub(crate) struct UnusedMembers {}

impl UnusedMembers {
    pub(crate) fn build_compilation_unit() -> Result<Rc<CompilationUnit>> {
        let mut builder = CompilationBuilder::create(LanguageFacts::LATEST_VERSION, Self {})?;

        builder.add_file(MAIN_ID)?;

        Ok(Rc::new(builder.build()))
    }
}

impl CompilationBuilderConfig for UnusedMembers {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> Result<Option<String>> {
        match file_id {
            MAIN_ID => Ok(Some(MAIN_SOL_CONTENTS.to_owned())),
            _ => Ok(None),
        }
    }

    fn resolve_import(
        &mut self,
        _source_file_id: &str,
        _import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> Result<Option<String>> {
        Ok(None)
    }
}
