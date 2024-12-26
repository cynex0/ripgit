use crate::core::repository::Repository;
use anyhow::{anyhow, Result};
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;
use std::str::from_utf8;
use std::{fs::File, io::Read};

pub trait GitObject {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self
    where
        Self: Sized;
    fn type_str(&self) -> &str;

    fn read_object(repo: &Repository, sha: &str) -> Result<Self>
    where
        Self: Sized,
    {
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

        Ok(Self::deserialize(body))
    }

    fn write_object(&self, repo: &Repository) -> Result<String> {
        let data = self.serialize();
        let header = format!("{} {}\0", self.type_str(), data.len());
        let obj = [header.as_bytes(), &data].concat();

        let mut hasher = Sha1::new(); // if this is highlighted as error, ignore it
        hasher.update(&obj);
        let hash = format!("{:x}", hasher.finalize());

        let object_dir = repo.gitdir.join("objects").join(&hash[..2]);
        let object_path = object_dir.join(&hash[2..]);

        fs::create_dir_all(&object_dir)?;
        let mut file = File::create(&object_path)?;
        let mut encoder =
            flate2::write::ZlibEncoder::new(&mut file, flate2::Compression::default());
        encoder.write_all(&obj)?;

        Ok(hash)
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
