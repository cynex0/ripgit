use std::path::{Path, PathBuf};

pub fn repo_file<P>(path: P, file_name: &str) -> PathBuf
where
    P: AsRef<Path>,
{
    PathBuf::from(path.as_ref()).join(file_name)
}
