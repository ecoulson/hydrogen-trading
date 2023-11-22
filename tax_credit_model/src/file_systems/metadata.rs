use std::fs::Metadata;

#[derive(Default, Debug, Eq, PartialEq)]
pub struct FileMetadata {
    size: u64,
}

impl FileMetadata {
    pub fn new(size: u64) -> Self {
        Self { size }
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn from_metadata(metadata: &Metadata) -> Self {
        Self::new(metadata.len())
    }
}

