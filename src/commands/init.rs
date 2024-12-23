use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

use crate::core::config::Config;
/// Creates a .git directory at the specified path
pub fn run_init<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().join(".git");
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
    use std::path::PathBuf;
    use tempdir::TempDir;

    #[test]
    fn should_create_git_dir() {
        let temp_dir = TempDir::new("init_test").unwrap();

        let res = run_init(&PathBuf::from(temp_dir.path()));
        assert!(res.is_ok());
        assert!(temp_dir.path().join(".git").exists());
    }
}
