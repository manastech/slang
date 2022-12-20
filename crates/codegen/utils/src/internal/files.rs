use std::path::PathBuf;

use anyhow::{bail, Context, Result};

use crate::{
    commands::run_command, context::CodegenContext, internal::formatting::format_source_file,
};

pub fn read_file(file_path: &PathBuf) -> Result<String> {
    return std::fs::read_to_string(file_path)
        .context(format!("Cannot read source file: {file_path:?}"));
}

pub fn delete_file(file_path: &PathBuf) -> Result<()> {
    return std::fs::remove_file(&file_path)
        .context(format!("Failed to delete extra file: {file_path:?}"));
}

pub fn write_file(codegen: &CodegenContext, file_path: &PathBuf, contents: &str) -> Result<()> {
    let parent_dir = file_path
        .parent()
        .context(format!("Cannot get parent directory of: {file_path:?}"))?;

    std::fs::create_dir_all(parent_dir)
        .context(format!("Cannot create parent directory of: {file_path:?}"))?;

    let formatted = format_source_file(codegen, file_path, &contents)?;

    // To respect Cargo incrementability, don't touch the file if it is already up to date.
    if file_path.exists() {
        let existing_contents = std::fs::read_to_string(file_path)
            .context(format!("Cannot read existing file: {file_path:?}"))?;

        if formatted == existing_contents {
            return Ok(());
        }
    }

    return std::fs::write(file_path, formatted)
        .context(format!("Cannot write generated file: {file_path:?}"));
}

pub fn verify_file(codegen: &CodegenContext, file_path: &PathBuf, contents: &str) -> Result<()> {
    let formatted = format_source_file(codegen, file_path, &contents)?;

    if !file_path.exists() {
        bail!("Generated file does not exist: {file_path:?}");
    }

    let existing_contents = std::fs::read_to_string(file_path)
        .context(format!("Cannot read existing file: {file_path:?}"))?;

    similar_asserts::assert_eq!(
        formatted,
        existing_contents,
        "Generated file is out of date: {file_path:?}"
    );

    return Ok(());
}

pub fn calculate_repo_root() -> Result<PathBuf> {
    if let Ok(repo_root) = std::env::var("REPO_ROOT") {
        return Ok(PathBuf::from(repo_root));
    }

    // If this environment variable is not set, it means we are running outside the Hermit environment.
    // Most likely, it is the rust-analyzer extension running inside VSCode.
    // Try to locate the repository root by looking for the ".git" directory.

    let git_dir = run_command(
        &std::env::current_dir()?,
        &["git", "rev-parse", "--git-dir"],
        None,
    )?;

    let git_dir = PathBuf::from(git_dir);
    let repo_root = git_dir.parent().unwrap().canonicalize()?;

    return Ok(repo_root);
}