use anyhow::{anyhow, Result};
use std::str::from_utf8;
use std::{fs::File, io::Read};

use super::repository::Repository;

pub enum GitObject {
    Blob { data: Vec<u8> },
    Commit {},
    Tree {},
    Tag {},
}

fn read_object(repo: Repository, sha: &str) -> Result<GitObject> {
    let path = repo.gitdir.join("objects").join(&sha[..2]).join(&sha[2..]);
    if !path.exists() {
        return Err(anyhow!("Object does not exist!"));
    }

    let file = File::open(path)?;
    let mut decoder = flate2::read::ZlibDecoder::new(file);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;
    let (header, body) = parse_git_obj(&data)?;
    let (obj_type, data_len) = parse_header(header)?;

    if body.len() != data_len {
        return Err(anyhow!("Invalid object body"));
    }

    match obj_type {
        "blob" => {
            return Ok(GitObject::Blob {
                data: Vec::from(body),
            })
        }
        "commit" => todo!(),
        "tree" => todo!(),
        "tag" => todo!(),
        _ => Err(anyhow!("Invalid object type in object file!")),
    }
}

fn parse_header(header: &[u8]) -> Result<(&str, usize)> {
    let (obj_type, len) = header.split_at(header.iter().position(|b| *b == 32).unwrap());
    let obj_type = from_utf8(obj_type)?;
    let len = from_utf8(&len[1..]).unwrap().parse().unwrap();
    Ok((obj_type, len))
}

fn parse_git_obj(data: &[u8]) -> Result<(&[u8], &[u8])> {
    if let Some(null_pos) = data.iter().position(|b| *b == 0) {
        let (header, body) = data.split_at(null_pos);
        Ok((header, &body[1..]))
    } else {
        Err(anyhow!("Malformatted object!"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_test() {
        if let Some(repo) = Repository::find(".") {
            let sha = "01f171672cfeacd66ae058720e093cd915e5e2f2"; // this is an object from the actual repo. probably should be some other way to test
            let obj = read_object(repo, sha).unwrap();
            match obj {
                GitObject::Blob { data } => println!("{:?}", from_utf8(&data).unwrap()),
                _ => assert!(false),
            }
        }
    }
}
