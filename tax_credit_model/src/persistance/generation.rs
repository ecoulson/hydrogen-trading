use serde_json::{from_str, to_string};

use crate::{
    files::file_system::{
        read_file, write_file, CreateMode, File, Permissions, ReadMode, WriteMode, delete_file,
    },
    schema::{
        errors::{Error, Result},
        simulation_schema::GenerationMetric,
    },
};

pub trait GenerationPersistanceClient {
    fn list_generations(&self) -> Result<Vec<GenerationMetric>>;
    fn create_generation(&self, generation_metric: &GenerationMetric) -> Result<GenerationMetric>;
    fn remove_all_generations(&self) -> Result<()>;
}

pub struct DiskGenerationPersistanceClient {
    file: File,
}

impl DiskGenerationPersistanceClient {
    pub fn new(path: &str) -> DiskGenerationPersistanceClient {
        DiskGenerationPersistanceClient {
            file: File::new(
                path,
                &Permissions::new(
                    ReadMode::Enabled,
                    WriteMode::Append(CreateMode::CreateOrRead),
                ),
            ),
        }
    }
}

impl GenerationPersistanceClient for DiskGenerationPersistanceClient {
    fn list_generations(&self) -> Result<Vec<GenerationMetric>> {
        let data = String::from_utf8(read_file(&self.file)?)
            .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))?;

        data.lines()
            .map(|line| {
                from_str(line).map_err(|err| Error::create_invalid_argument_error(&err.to_string()))
            })
            .collect()
    }

    fn create_generation(&self, generation_metric: &GenerationMetric) -> Result<GenerationMetric> {
        write_file(
            &self.file,
            &to_string(generation_metric)
                .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))?
                .bytes()
                .collect::<Vec<u8>>(),
        )?;

        Ok(generation_metric.clone())
    }

    fn remove_all_generations(&self) -> Result<()> {
        write_file(
            &File::new(
                self.file.path(),
                &Permissions::writeable(CreateMode::CreateOrRead),
            ),
            &vec![],
        )?;

        Ok(())
    }
}
