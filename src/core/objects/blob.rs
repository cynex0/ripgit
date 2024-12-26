use crate::core::objects::gitobject::GitObject;
pub struct Blob {
    pub blobdata: Vec<u8>,
}

impl GitObject for Blob {
    fn serialize(&self) -> Vec<u8> {
        self.blobdata.clone()
    }

    fn deserialize(data: &[u8]) -> Self {
        Blob {
            blobdata: data.to_vec(),
        }
    }

    fn type_str(&self) -> &str {
        "blob"
    }
}

#[cfg(test)]
mod blob_tests {
    use super::*;
    use crate::commands::init::run_init;
    use crate::core::repository::Repository;
    use tempdir::TempDir;

    fn create_blob() -> Blob {
        Blob {
            blobdata: "Hello, world!".as_bytes().to_vec(),
        }
    }

    #[test]
    fn blob_roundtrip() {
        let temp_dir = TempDir::new("init_test").unwrap();
        if let Err(_) = run_init(temp_dir.path()) {
            assert!(false);
        }
        let repo = Repository::new(&temp_dir.path()).unwrap();

        let blob = create_blob();
        if let Ok(sha) = blob.write_object(Some(&repo)) {
            let new_blob = Blob::read_object(&repo, &sha).unwrap();
            let string = String::from_utf8(new_blob.blobdata.clone()).unwrap();
            assert_eq!(blob.blobdata, new_blob.blobdata);
            assert_eq!("Hello, world!", string);
        } else {
            assert!(false);
        }
    }
}
