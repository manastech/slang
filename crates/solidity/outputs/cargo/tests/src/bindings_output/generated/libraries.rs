// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn constants() -> Result<()> {
    run("libraries", "constants")
}

#[test]
fn visibility() -> Result<()> {
    run("libraries", "visibility")
}
