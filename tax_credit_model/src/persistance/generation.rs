use std::sync::Arc;

use serde_json::{from_str, to_string};

use crate::{
    files::file_system::{
        create_directory, read_file, write_file, CreateMode, File, Permissions, ReadMode, WriteMode,
    },
    schema::{
        errors::{Error, Result},
        simulation_schema::GenerationMetric,
    },
};

pub trait GenerationClient: Send + Sync {
    fn list_generations(&self) -> Result<Vec<GenerationMetric>>;
    fn create_generation(&self, generation_metric: &GenerationMetric) -> Result<GenerationMetric>;
    fn remove_all_generations(&self) -> Result<()>;
}

pub struct DiskGenerationPersistanceClient {
    file: Arc<File>,
}

impl DiskGenerationPersistanceClient {
    pub fn new(path: &str) -> Result<DiskGenerationPersistanceClient> {
        let data_file = File::new(path, &Permissions::appendable(CreateMode::CreateOrRead));
        create_directory(File::directory_path(&data_file))?;
        write_file(&data_file, &vec![])?;

        Ok(DiskGenerationPersistanceClient {
            file: Arc::new(File::new(
                path,
                &Permissions::new(
                    ReadMode::Enabled,
                    WriteMode::Append(CreateMode::CreateOrRead),
                ),
            )),
        })
    }
}

impl GenerationClient for DiskGenerationPersistanceClient {
    fn list_generations(&self) -> Result<Vec<GenerationMetric>> {
        let data = String::from_utf8(read_file(&self.file)?)
            .map_err(|err| Error::invalid_argument(&err.to_string()))?;

        data.lines()
            .map(|line| {
                from_str(line).map_err(|err| Error::invalid_argument(&err.to_string()))
            })
            .collect()
    }

    fn create_generation(&self, generation_metric: &GenerationMetric) -> Result<GenerationMetric> {
        write_file(
            &self.file,
            &format!(
                "{}\n",
                to_string(generation_metric)
                    .map_err(|err| Error::invalid_argument(&err.to_string()))?
            )
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
