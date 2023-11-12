mod utils;

use std::{
    fs::{self, create_dir_all, metadata},
    path::Path,
};

use tax_credit_model_server::files::file_system::{
    create_directory, delete_file, file_metadata, read_file, write_file, CreateMode, Directory,
    File, FileMetadata, Permissions, ReadMode, WriteMode,
};

use utils::temp_dir::TempDirectory;

#[test]
fn write_file_to_disk() {
    let directory =
        TempDirectory::create_from_env("TMPDIR", "write").expect("Failed to create temp dir");
    let expected_file = File::new(
        &TempDirectory::canonicalize_path(&directory, "file.txt"),
        &Permissions::default(),
    );
    let input_file = File::new(
        &TempDirectory::canonicalize_path(&directory, "file.txt"),
        &Permissions::writeable(CreateMode::CreateOnly),
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
        &TempDirectory::canonicalize_path(&directory, "file.txt"),
        &Permissions::readable(),
    );
    let expected_data = &"Hello world!".bytes().collect::<Vec<u8>>();
    std::fs::write(input_file.path(), expected_data).expect("Failed to write test data");
    let stored_data = fs::read(input_file.path()).expect("Failed to read written data");

    let actual_data = read_file(&input_file).expect("Failed to read whole file");

    assert_eq!(expected_data, &stored_data);
    assert_eq!(&actual_data, expected_data);
}

#[test]
fn successfully_remove_file() {
    let directory =
        TempDirectory::create_from_env("TMPDIR", "delete").expect("Failed to create temp dir");
    let input_file = File::new(
        &TempDirectory::canonicalize_path(&directory, "file.txt"),
        &Permissions::writeable(CreateMode::CreateOrRead),
    );
    let data = &"Hello world!".bytes().collect::<Vec<u8>>();
    std::fs::write(input_file.path(), data).expect("Failed to write test data");
    let stored_data = fs::read(input_file.path()).expect("Failed to read written data");

    delete_file(&input_file).expect("Failed to read whole file");

    assert_eq!(data, &stored_data);
    assert!(!Path::new(input_file.path())
        .try_exists()
        .expect("File should not exist"));
}

#[test]
fn should_get_metadata() {
    let directory =
        TempDirectory::create_from_env("TMPDIR", "metadata").expect("Failed to create temp dir");
    let input_file = File::new(
        &TempDirectory::canonicalize_path(&directory, "file.txt"),
        &Permissions::new(
            ReadMode::Enabled,
            WriteMode::Append(CreateMode::CreateOrRead),
        ),
    );
    let data = &"Hello world!".bytes().collect::<Vec<u8>>();
    let expected_metadata = FileMetadata::new(data.len() as u64);
    std::fs::write(input_file.path(), data).expect("Failed to write test data");

    let actual_metadata = file_metadata(&input_file).unwrap();

    assert_eq!(actual_metadata, expected_metadata);
}

#[test]
fn should_create_directory() {
    let root_dir =
        TempDirectory::create_from_env("TMPDIR", "create_dir").expect("Failed to create temp dir");
    let directory_path = TempDirectory::canonicalize_path(&root_dir, "create_dir");
    let expected_directory = Directory::new(&directory_path);

    let directory = create_directory(&directory_path).unwrap();
    let directory_metadata = metadata(expected_directory.path()).unwrap();

    assert!(directory_metadata.is_dir());
    assert_eq!(directory, expected_directory);
}

#[test]
fn create_existing_directory() {
    let root_dir = TempDirectory::create_from_env("TMPDIR", "existing_dir")
        .expect("Failed to create temp dir");
    let directory_path = TempDirectory::canonicalize_path(&root_dir, "existing_dir");
    let expected_directory = Directory::new(&directory_path);
    create_dir_all(expected_directory.path()).unwrap();

    let directory = create_directory(&directory_path).unwrap();
    let directory_metadata = metadata(expected_directory.path()).unwrap();

    assert!(directory_metadata.is_dir());
    assert_eq!(directory, expected_directory);
}

