use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::core::config::Config;
/// Creates a .git directory at the specified path
pub fn run_init(path: &PathBuf) -> Result<()> {
    let path = path.join(".git");
    if path.exists() {
        return Err(anyhow!("Already a git repository"));
    }

    fs::create_dir_all(&path).with_context(|| "Failed to create .git directory")?;
    Config::new().write(&path)?;
    Ok(())
}

#[cfg(test)]
mod init_tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn should_create_git_dir() {
        let temp_dir = TempDir::new("init_test").unwrap();

        let res = run_init(&PathBuf::from(temp_dir.path()));
        assert!(res.is_ok());
        assert!(temp_dir.path().join(".git").exists());
    }
}
