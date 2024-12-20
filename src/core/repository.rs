use crate::core::config::Config;
use std::path::{Path, PathBuf};

pub struct Repository {
    pub gitdir: PathBuf,
    pub worktreee: PathBuf,
    pub config: Config,
}

impl Repository {
    /// Creates a Repository object representing a repo located at a given path
    /// # Returns
    /// Ok if the given path contains a valid .git
    ///
    /// Err otherwise
    pub fn new<P>(path: P) -> Result<Repository, &'static str>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().join(".git");
        if !path.exists() {
            Err("Not a git repository")
        } else {
            let config = Config::new();
            Ok(Repository {
                gitdir: path.clone(),
                worktreee: path.parent().unwrap().to_path_buf(),
                config,
            })
        }
    }

    pub fn find<P>(path: P) -> Option<Repository>
    where
        P: AsRef<Path>,
    {
        for p in path.as_ref().ancestors() {
            if let Ok(repo) = Repository::new(p) {
                return Some(repo);
            }
        }
        None
    }
}

#[cfg(test)]
mod repo_tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn should_find_in_same_dir() {
        let orig_dir = env::current_dir().unwrap();
        let temp_dir = TempDir::new("find_test_1").unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        let gitdir = temp_dir.path().join(".git");
        fs::create_dir(&gitdir).unwrap();
        assert!(gitdir.exists());

        if let Some(repo) = Repository::find(&temp_dir) {
            assert_eq!(repo.gitdir, gitdir);
        } else {
            assert!(false);
        }

        env::set_current_dir(orig_dir).unwrap();
    }

    #[test]
    fn should_find_in_nested_dir() {
        let orig_dir = env::current_dir().unwrap();
        let temp_dir = TempDir::new("find_test_2").unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        let gitdir = temp_dir.path().join(".git");
        fs::create_dir(&gitdir).unwrap();
        assert!(gitdir.exists());

        if let Some(repo) = Repository::find(&temp_dir.path().join("foo/bar/rust/is/cool")) {
            assert_eq!(repo.gitdir, gitdir);
        } else {
            assert!(false);
        }

        env::set_current_dir(orig_dir).unwrap();
    }

    #[test]
    fn should_not_find_if_not_exists() {
        let orig_dir = env::current_dir().unwrap();
        let temp_dir = TempDir::new("find_test_3").unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        assert!(!temp_dir.path().join(".git").exists());

        if let Some(_) = Repository::find(&temp_dir.path().join("foo/bar/rust/is/cool")) {
            assert!(false);
        }

        env::set_current_dir(orig_dir).unwrap();
    }

    #[test]
    fn should_find_first() {
        let orig_dir = env::current_dir().unwrap();
        let temp_dir = TempDir::new("find_test_3").unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        let target_path = temp_dir.path().join("1/2/.git");
        fs::create_dir_all(temp_dir.path().join("1/.git")).unwrap();
        fs::create_dir_all(&target_path).unwrap();
        assert!(target_path.exists());
        assert!(temp_dir.path().join("1/.git").exists());

        if let Some(p) = Repository::find(target_path.parent().unwrap().join("3")) {
            assert_eq!(p.gitdir, target_path);
        } else {
            assert!(false);
        }

        env::set_current_dir(orig_dir).unwrap();
    }
}
