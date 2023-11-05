mod utils;

use std::{
    fs::{self},
    path::Path,
};

use tax_credit_model_server::files::file_system::{
    delete_file, read_file, write_file, CreateMode, Permissions, file_metadata, FileMetadata,
};

use utils::temp_dir::TempDirectory;

#[test]
fn write_file_to_disk() {
    let directory =
        TempDirectory::create_from_env("TMPDIR", "write").expect("Failed to create temp dir");
    let expected_file = TempDirectory::create_file(&directory, "file.txt", &Permissions::default());
    let input_file = TempDirectory::create_file(
        &directory,
        "file.txt",
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
    let input_file = TempDirectory::create_file(&directory, "file.txt", &Permissions::readable());
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
    let input_file = TempDirectory::create_file(
        &directory,
        "file.txt",
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
    let input_file = TempDirectory::create_file(
        &directory,
        "file.txt",
        &Permissions::appendable(CreateMode::CreateOrRead),
    );
    let data = &"Hello world!".bytes().collect::<Vec<u8>>();
    let expected_metadata = FileMetadata::new(data.len() as u64);
    std::fs::write(input_file.path(), data).expect("Failed to write test data");

    let actual_metadata = file_metadata(&input_file).unwrap();

    assert_eq!(actual_metadata, expected_metadata);
}
