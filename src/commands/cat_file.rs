use crate::core::object::{read_object, GitObject};
use crate::core::repository::Repository;
use std::env;
use std::path::Path;

pub fn run_cat_file<P>(path: P, sha: &str)
where
    P: AsRef<Path>,
{
    // TODO: object search, name resolution
    let repo = Repository::find(path.as_ref()).unwrap();
    if let Ok(obj) = read_object(repo, sha) {
        match obj {
            GitObject::Blob { data } => {
                println!("{}", String::from_utf8(data).unwrap());
            }
            GitObject::Commit {} => {
                todo!();
            }
            GitObject::Tree {} => {
                todo!();
            }
            GitObject::Tag {} => {
                todo!();
            }
        }
    } else {
        println!("Failed to read object");
    }
}
