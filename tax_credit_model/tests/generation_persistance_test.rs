mod utils;

use tax_credit_model_server::{
    files::file_system::{
        file_metadata, write_file, CreateMode, Permissions, ReadMode, WriteMode,
    },
    persistance::generation::{DiskGenerationPersistanceClient, GenerationPersistanceClient},
    schema::{
        simulation_schema::{EnergySourcePortfolio, GenerationMetric},
        time::Timestamp,
    },
};
use utils::temp_dir::TempDirectory;

#[test]
fn list_generations_from_disk() {
    let directory = TempDirectory::create_from_env("TMPDIR", "list_generations").unwrap();
    let file = TempDirectory::create_file(
        &directory,
        "generations.data",
        &Permissions::writeable(CreateMode::CreateOnly),
    );
    let client = DiskGenerationPersistanceClient::new(file.path());
    let expected_generations = vec![GenerationMetric::new(
        0,
        &Timestamp::new(0, 0),
        0.0,
        EnergySourcePortfolio::default(),
    )];
    let serialized_generation = serde_json::to_string(&GenerationMetric::default())
        .unwrap()
        .bytes()
        .collect::<Vec<u8>>();
    write_file(&file, &serialized_generation).unwrap();

    let generations = client.list_generations().unwrap();

    assert_eq!(generations, expected_generations);
}

#[test]
fn create_generations_on_disk() {
    let directory = TempDirectory::create_from_env("TMPDIR", "create_generations").unwrap();
    let file = TempDirectory::create_file(
        &directory,
        "generations.data",
        &Permissions::writeable(CreateMode::CreateOnly),
    );
    let client = DiskGenerationPersistanceClient::new(file.path());
    let input_generation = GenerationMetric::new(
        0,
        &Timestamp::new(0, 0),
        0.0,
        EnergySourcePortfolio::default(),
    );
    let expected_generation = &input_generation;

    let actual_generation = client.create_generation(&input_generation).unwrap();

    assert_eq!(&actual_generation, expected_generation);
}

#[test]
fn delete_all_generations_on_disk() {
    let directory = TempDirectory::create_from_env("TMPDIR", "delete_all_generations").unwrap();
    let file = TempDirectory::create_file(
        &directory,
        "generations.data",
        &Permissions::new(
            ReadMode::Enabled,
            WriteMode::Overwrite(CreateMode::CreateOrRead),
        ),
    );
    let client = DiskGenerationPersistanceClient::new(file.path());
    let serialized_generation = serde_json::to_string(&GenerationMetric::default())
        .unwrap()
        .bytes()
        .collect::<Vec<u8>>();
    write_file(&file, &serialized_generation).unwrap();
    let metadata = file_metadata(&file).unwrap();
    assert_ne!(metadata.size(), 0);

    client.remove_all_generations().unwrap();
    let metadata = file_metadata(&file).unwrap();

    assert_eq!(metadata.size(), 0);
}
