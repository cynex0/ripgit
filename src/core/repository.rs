use crate::core::config::Config;
use anyhow::{anyhow, Context, Result};
use std::fs;
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
        let config = Config::new();

        let path = path.as_ref().join(".git");
        if !path.exists() {
            Err("Not a git repository")
        } else {
            Ok(Repository {
                gitdir: path.clone(),
                worktreee: path.parent().unwrap().to_path_buf(),
                config,
            })
        }
    }

    /// Creates a new repository directory at specified path.
    pub fn init<P>(path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().join(".git");
        if !path.exists() {
            fs::create_dir_all(path).with_context(|| "Failed to create .git directory")?;
            Ok(())
        } else {
            Err(anyhow!("Already a git repository"))
        }
    }
}
