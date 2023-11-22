use std::fs::create_dir_all;

use crate::schema::errors::{Result, Error};


#[derive(Default, Debug, Eq, PartialEq)]
pub struct Directory {
    path: String,
}

impl Directory {
    pub fn new(path: &str) -> Directory {
        Directory {
            path: String::from(path),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn create_directory(path: &str) -> Result<Directory> {
        create_dir_all(path).map_err(|err| Error::invalid_argument(&err.to_string()))?;

        Ok(Directory::new(path))
    }
}

