use std::io::{Read, Write};

use crate::schema::errors::{Error, Result};

use super::{metadata::FileMetadata, permission::Permissions};

#[derive(Default, Debug, Eq, PartialEq)]
pub struct File {
    path: String,
    permissions: Permissions,
}

impl File {
    pub fn new(path: &str, permissions: &Permissions) -> Self {
        Self {
            path: String::from(path),
            permissions: permissions.clone(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn permissions(&self) -> &Permissions {
        &self.permissions
    }

    pub fn directory_path(&self) -> &str {
        for (i, char) in self.path.chars().rev().enumerate() {
            if char == '/' {
                return &self.path[0..self.path().len() - i - 1];
            }
        }

        "/"
    }

    pub fn read_file(&self) -> Result<Vec<u8>> {
        if !self.permissions().can_read() {
            return Err(Error::invalid_argument("File must be readable"));
        }

        let mut buffer = Vec::new();
        Permissions::readable()
            .open_file_handle(self.path())?
            .read_to_end(&mut buffer)
            .map_err(|err| Error::invalid_argument(&err.to_string()))?;

        Ok(buffer)
    }

    pub fn write_file<'f>(&'f self, content: &[u8]) -> Result<&'f Self> {
        if !Permissions::can_write(self.permissions()) {
            return Err(Error::invalid_argument("File must be writeable"));
        }

        self.permissions()
            .open_file_handle(self.path())?
            .write_all(content)
            .map_err(|err| Error::invalid_argument(&err.to_string()))?;

        Ok(self)
    }

    pub fn delete_file(&self) -> Result<&Self> {
        std::fs::remove_file(self.path())
            .map_err(|err| Error::invalid_argument(&err.to_string()))?;

        Ok(self)
    }

    pub fn file_metadata(&self) -> Result<FileMetadata> {
        if !Permissions::can_read(self.permissions()) {
            return Err(Error::invalid_argument("File must be readable"));
        }

        let metadata = Permissions::readable()
            .open_file_handle(self.path())?
            .metadata()
            .map_err(|err| Error::invalid_argument(&err.to_string()))?;

        Ok(FileMetadata::from_metadata(&metadata))
    }
}

#[cfg(test)]
mod test {
    use crate::file_systems::file::{File, Permissions};

    #[test]
    fn parse_directory_path() {
        let input_file = File::new("foo/bar/baz/file.txt", &Permissions::readable());
        let expected_directory = "foo/bar/baz";

        let directory_path = File::directory_path(&input_file);

        assert_eq!(directory_path, expected_directory);
    }

    #[test]
    fn parse_root_directory_path() {
        let input_file = File::new("file.txt", &Permissions::readable());
        let expected_directory = "/";

        let directory_path = File::directory_path(&input_file);

        assert_eq!(directory_path, expected_directory);
    }
}
