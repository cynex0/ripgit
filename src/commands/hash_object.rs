use anyhow::Result;
use std::path::Path;

use crate::core::objects::blob::Blob;
use crate::core::objects::gitobject::GitObject;
use crate::core::repository::Repository;
use crate::utils::git_object_types::GitObjectTypes;

pub fn run_hash_object<P>(path: P, type_: GitObjectTypes) -> Result<String>
where
    P: AsRef<Path>,
{
    match type_ {
        GitObjectTypes::Blob => {
            let repo = Repository::find(path.as_ref()).unwrap();
            let blob = Blob::from_file(&repo, path)?;
            let hash = blob.write_object(None)?;
            return Ok(hash);
        }
        _ => unimplemented!(),
    }
}
