use std::fs::{self, remove_dir_all};

use tax_credit_model_server::{
    files::file_system::{File, Permissions, Directory},
    schema::errors::{Error, Result},
};

#[derive(Debug)]
pub struct TempDirectory {
    path: String,
}

impl TempDirectory {
    pub fn create_from_env(temporary_directory_env_key: &str, path: &str) -> Result<TempDirectory> {
        let temp_dir_path = std::env::var(temporary_directory_env_key).expect("No tmp dir exists");
        let path = format!("{}hydrogen/{}", temp_dir_path, path);

        match fs::create_dir_all(&path) {
            Ok(_) => Ok(TempDirectory { path }),
            Err(err) => match err.kind() {
                std::io::ErrorKind::AlreadyExists => Ok(TempDirectory { path }),
                _ => Err(Error::create_invalid_argument_error(
                    "Failed to create tmp dir",
                )),
            },
        }
    }

    pub fn canonicalize_path(directory: &TempDirectory, path: &str) -> String {
        format!("{}/{}", directory.path, path)
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        remove_dir_all(&self.path).expect("Failed to remove path");
    }
}
