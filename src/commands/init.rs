use crate::core::repository::Repository;
use anyhow::Result;
use std::path::Path;

pub fn run_init<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    Repository::init(&path)
}

#[cfg(test)]
mod init_tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn should_create_git_dir() {
        let temp_dir = TempDir::new("init_test").unwrap();

        let res = run_init(temp_dir.path());
        assert!(res.is_ok());
        assert!(temp_dir.path().join(".git").exists());
    }
}
