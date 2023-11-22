use std::sync::Arc;

use serde_json::{from_str, to_string};

use crate::{
    file_systems::{
        directory::Directory,
        file::File,
        permission::{CreateMode, Permissions, ReadMode, WriteMode},
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
    pub fn new(path: &str) -> Result<Self> {
        let data_file = File::new(path, &Permissions::appendable(CreateMode::CreateOrRead));
        Directory::create_directory(File::directory_path(&data_file))?;
        data_file.write_file(&vec![])?;

        Ok(Self {
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
        let data = String::from_utf8(self.file.read_file()?)
            .map_err(|err| Error::invalid_argument(&err.to_string()))?;

        data.lines()
            .map(|line| from_str(line).map_err(|err| Error::invalid_argument(&err.to_string())))
            .collect()
    }

    fn create_generation(&self, generation_metric: &GenerationMetric) -> Result<GenerationMetric> {
        self.file.write_file(
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
        let empty_file = &File::new(
            self.file.path(),
            &Permissions::writeable(CreateMode::CreateOrRead),
        );
        empty_file.write_file(&vec![])?;

        Ok(())
    }
}
