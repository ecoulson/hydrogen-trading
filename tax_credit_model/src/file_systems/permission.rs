use std::fs::OpenOptions;

use crate::schema::errors::{Error, Result};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum WriteMode {
    #[default]
    Disabled,
    Overwrite(CreateMode),
    Append(CreateMode),
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum CreateMode {
    #[default]
    CreateOnly,
    CreateOrRead,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum ReadMode {
    #[default]
    Disabled,
    Enabled,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Permissions {
    read: ReadMode,
    write: WriteMode,
}

impl Permissions {
    pub fn readable() -> Permissions {
        Permissions {
            read: ReadMode::Enabled,
            write: WriteMode::Disabled,
        }
    }

    pub fn writeable(create_mode: CreateMode) -> Permissions {
        Permissions {
            read: ReadMode::Disabled,
            write: WriteMode::Overwrite(create_mode),
        }
    }

    pub fn appendable(create_mode: CreateMode) -> Permissions {
        Permissions {
            read: ReadMode::Disabled,
            write: WriteMode::Append(create_mode),
        }
    }

    pub fn new(read: ReadMode, write: WriteMode) -> Self {
        Self { read, write }
    }

    pub fn can_write(&self) -> bool {
        !matches!(self.write, WriteMode::Disabled)
    }

    pub fn can_read(&self) -> bool {
        return matches!(self.read, ReadMode::Enabled);
    }

    pub fn open_file_handle(&self, path: &str) -> Result<std::fs::File> {
        self.create_open_options()
            .open(path)
            .map_err(|err| Error::invalid_argument(&err.to_string()))
    }

    fn create_open_options(&self) -> OpenOptions {
        let mut open_options = OpenOptions::new();
        open_options.write(true);

        match self.read {
            ReadMode::Enabled => open_options.read(true),
            ReadMode::Disabled => open_options.read(false),
        };
        match &self.write {
            WriteMode::Disabled => open_options.write(false),
            WriteMode::Append(create_mode) => {
                open_options.append(true);
                Permissions::set_create_open_options(&create_mode, &mut open_options)
            }
            WriteMode::Overwrite(create_mode) => {
                open_options.truncate(true);
                Permissions::set_create_open_options(&create_mode, &mut open_options)
            }
        };

        open_options
    }

    fn set_create_open_options<'a>(
        mode: &'a CreateMode,
        open_options: &'a mut OpenOptions,
    ) -> &'a OpenOptions {
        match mode {
            CreateMode::CreateOnly => open_options.create_new(true),
            CreateMode::CreateOrRead => open_options.create(true),
        }
    }
}
