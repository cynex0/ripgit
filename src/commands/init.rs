use crate::core::repository::Repository;
use std::fs;
use std::path::Path;

pub fn run_init<P>(path: P) -> Result<Repository, &'static str>
where
    P: AsRef<Path>,
{
    let gitdir = path.as_ref().join(".git");

    if !gitdir.exists() {
        fs::create_dir_all(gitdir).unwrap();
    } else {
        println!("Already a git repository");
    }

    Repository::new(path)
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
        let repo = res.unwrap();
        assert_eq!(repo.gitdir, temp_dir.path().join(".git"));
        assert!(repo.gitdir.exists());
    }
}
