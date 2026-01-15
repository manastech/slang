use anyhow::Result;

mod counter;
mod overrides;
mod unused_members;

pub(crate) use counter::Counter;
pub(crate) use overrides::Overrides;
pub(crate) use unused_members::UnusedMembers;

#[test]
fn test_build_counter_fixture() -> Result<()> {
    let unit = Counter::build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(3, semantic_analysis.files().len());

    Ok(())
}

#[test]
fn test_build_overrides_fixture() -> Result<()> {
    let unit = Overrides::build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(1, semantic_analysis.files().len());

    Ok(())
}

#[test]
fn test_build_unused_members_fixture() -> Result<()> {
    let unit = UnusedMembers::build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(1, semantic_analysis.files().len());

    Ok(())
}
