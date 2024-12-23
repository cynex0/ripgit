use crate::core::config::Config;
use anyhow::{anyhow, Result};
use std::{
    path::{Path, PathBuf},
    usize::MAX,
};

use super::config::CoreSection;

pub struct Repository {
    pub gitdir: PathBuf,
    pub worktree: PathBuf,
    pub config: Config,
}

impl Repository {
    /// Creates a Repository object representing a repo located at a given path
    /// # Returns
    /// Ok if the given path contains a valid .git
    ///
    /// Err otherwise
    pub fn new<P>(path: P) -> Result<Repository>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().join(".git");
        if !path.exists() {
            Err(anyhow!("Not a git repository"))
        } else {
            let config = Config::read(&path)?;
            Ok(Repository {
                gitdir: path.clone(),
                worktree: path.parent().unwrap().to_path_buf(),
                config,
            })
        }
    }

    pub fn find<P>(path: P) -> Option<Repository>
    where
        P: AsRef<Path>,
    {
        for p in path.as_ref().ancestors() {
            println!("Checking {:?}", p);
            match Repository::new(p) {
                Ok(repo) => return Some(repo),
                Err(e) => println!("Not a repo: {:?}", e),
            }
        }
        None
    }
}

#[cfg(test)]
mod repo_tests {
    use super::*;
    use crate::commands::init::run_init;
    use std::env;
    use tempdir::TempDir;

    fn create_repo<P: AsRef<Path>>(path: P) {
        if let Ok(_) = run_init(path.as_ref()) {
            println!("Initialized repo at {:?}", path.as_ref());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_find_in_same_dir() {
        let orig_dir = env::current_dir().unwrap();
        let temp_dir = TempDir::new("find_test_1").unwrap();
        let temp_dir = temp_dir.path();
        env::set_current_dir(&temp_dir).unwrap();

        create_repo(temp_dir);
        let gitdir = temp_dir.join(".git");
        assert!(gitdir.exists());
        println!("Created .git in {:?}", temp_dir);

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
        let temp_dir = temp_dir.path();
        env::set_current_dir(temp_dir).unwrap();

        create_repo(temp_dir);
        let gitdir = temp_dir.join(".git");
        assert!(gitdir.exists());

        if let Some(repo) = Repository::find(&temp_dir.join("foo/bar/rust/is/cool")) {
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
        let temp_dir = temp_dir.path();
        env::set_current_dir(temp_dir).unwrap();

        assert!(!temp_dir.join(".git").exists());

        if let Some(_) = Repository::find(&temp_dir.join("foo/bar/rust/is/cool")) {
            assert!(false);
        }

        env::set_current_dir(orig_dir).unwrap();
    }

    #[test]
    fn should_find_first() {
        let orig_dir = env::current_dir().unwrap();
        let temp_dir = TempDir::new("find_test_4").unwrap();
        let temp_dir = temp_dir.path();
        env::set_current_dir(temp_dir).unwrap();

        create_repo(temp_dir.join("1"));
        create_repo(temp_dir.join("1/2"));
        let target_path = temp_dir.join("1/2/.git");
        assert!(target_path.exists());
        assert!(temp_dir.join("1/.git").exists());

        if let Some(p) = Repository::find(target_path.parent().unwrap().join("3")) {
            assert_eq!(p.gitdir, target_path);
        } else {
            assert!(false);
        }

        env::set_current_dir(orig_dir).unwrap();
    }
}
