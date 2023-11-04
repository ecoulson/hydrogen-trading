use std::fs::{self, remove_dir_all};

use tax_credit_model_server::{
    files::file_system::{
        read_file, write_file, CreateMode, File, Permissions, ReadMode, WriteMode,
    },
    schema::errors::{Error, Result},
};

#[derive(Debug)]
struct TempDirectory {
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
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        remove_dir_all(&self.path).expect("Failed to remove path");
    }
}

#[test]
fn write_file_to_disk() {
    let directory =
        TempDirectory::create_from_env("TMPDIR", "write").expect("Failed to create temp dir");
    let expected_file = File::new(
        &format!("{}/{}", directory.path, "file.txt"),
        Permissions::default(),
    );
    let input_file = File::new(
        &format!("{}/{}", directory.path, "file.txt"),
        Permissions::new(
            ReadMode::Disabled,
            WriteMode::Overwrite(CreateMode::CreateOnly),
        ),
    );
    let data = &"Hello world!".bytes().collect::<Vec<u8>>();

    let resulting_file = write_file(&input_file, data).unwrap();

    let stored_data = fs::read(expected_file.path()).expect("Failed to read written data");
    assert_eq!(resulting_file, &input_file);
    assert_eq!(data, &stored_data);
}

#[test]
fn read_from_file() {
    let directory =
        TempDirectory::create_from_env("TMPDIR", "read").expect("Failed to create temp dir");
    let input_file = File::new(
        &format!("{}/{}", directory.path, "file.txt"),
        Permissions::new(ReadMode::Enabled, WriteMode::Disabled),
    );
    let expected_data = &"Hello world!".bytes().collect::<Vec<u8>>();
    std::fs::write(input_file.path(), expected_data).expect("Failed to write test data");
    let stored_data = fs::read(input_file.path()).expect("Failed to read written data");
    assert_eq!(expected_data, &stored_data);

    let actual_data = read_file(&input_file).expect("Failed to read whole file");

    assert_eq!(&actual_data, expected_data);
}
