use crate::core::config::Config;
use std::path::{Path, PathBuf};

pub struct Repository {
    pub gitdir: PathBuf,
    pub worktreee: PathBuf,
    pub config: Config,
}

impl Repository {
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
}
