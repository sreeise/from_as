use std::error::Error;
use std::path::Path;

pub trait FromFile<RHS = Self> {
    type Error: Error;

    fn from_file<P: AsRef<Path>>(path: P) -> Result<RHS, Self::Error>
    where
        for<'de> Self: serde::Deserialize<'de>;
}

pub trait AsFile {
    type Error: Error;

    fn as_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error>
    where
        Self: serde::Serialize;

    fn as_file_pretty<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error>
    where
        Self: serde::Serialize;
}
