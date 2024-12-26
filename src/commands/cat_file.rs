use crate::core::objects::blob::Blob;
use crate::core::objects::gitobject::GitObject;
use crate::core::repository::Repository;
use std::path::Path;

pub fn run_cat_file<P>(path: P, sha: &str)
where
    P: AsRef<Path>,
{
    // TODO: object search, name resolution
    let repo = Repository::find(path.as_ref()).unwrap();
    if let Ok(obj) = GitObject::read_object(&repo, sha) {
        match obj {
            Blob { blobdata } => {
                println!("{}", String::from_utf8(blobdata).unwrap());
            }
        }
    } else {
        println!("Failed to read object");
    }
}
