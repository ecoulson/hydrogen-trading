use std::{
    fs::{Metadata, OpenOptions},
    io::{Read, Write},
};

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

    pub fn new(read: ReadMode, write: WriteMode) -> Permissions {
        Permissions { read, write }
    }

    pub fn can_write(permissions: &Permissions) -> bool {
        !matches!(permissions.write, WriteMode::Disabled)
    }

    pub fn can_read(permissions: &Permissions) -> bool {
        return matches!(permissions.read, ReadMode::Enabled);
    }

    fn create_open_options(permissions: &Permissions) -> OpenOptions {
        let mut open_options = OpenOptions::new();
        open_options.write(true);

        match permissions.read {
            ReadMode::Enabled => open_options.read(true),
            ReadMode::Disabled => open_options.read(false),
        };
        match &permissions.write {
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

#[derive(Default, Debug, Eq, PartialEq)]
pub struct File {
    path: String,
    permissions: Permissions,
}

impl File {
    pub fn new(path: &str, permissions: &Permissions) -> File {
        File {
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
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct FileMetadata {
    size: u64,
}

impl FileMetadata {
    pub fn new(size: u64) -> FileMetadata {
        FileMetadata {
            size
        }
    }

    pub fn from_metadata(metadata: &Metadata) -> FileMetadata {
        FileMetadata {
            size: metadata.len(),
        }
    }
}

pub fn open_file_handle(file: &File) -> Result<std::fs::File> {
    Permissions::create_open_options(file.permissions())
        .open(file.path())
        .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))
}

pub fn read_file(file: &File) -> Result<Vec<u8>> {
    if !Permissions::can_read(file.permissions()) {
        return Err(Error::create_invalid_argument_error(
            "File must be readable",
        ));
    }

    let mut buffer = Vec::new();
    open_file_handle(file)?
        .read_to_end(&mut buffer)
        .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))?;

    Ok(buffer)
}

pub fn write_file<'f>(file: &'f File, content: &[u8]) -> Result<&'f File> {
    if !Permissions::can_write(file.permissions()) {
        return Err(Error::create_invalid_argument_error(
            "File must be writeable",
        ));
    }

    open_file_handle(file)?
        .write_all(content)
        .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))?;

    Ok(file)
}

pub fn delete_file(file: &File) -> Result<&File> {
    std::fs::remove_file(file.path())
        .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))?;

    Ok(file)
}

pub fn file_metadata(file: &File) -> Result<FileMetadata> {
    let metadata = open_file_handle(file)?
        .metadata()
        .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))?;

    Ok(FileMetadata::from_metadata(&metadata))
}
