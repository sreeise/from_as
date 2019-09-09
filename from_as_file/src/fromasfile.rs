use std::error::Error;
use std::path::Path;

pub trait FromFile<RHS = Self>
where
    for<'de> Self: serde::Deserialize<'de>,
{
    type Error: Error;

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error>;
}

pub trait AsFile<RHS = Self>
where
    Self: serde::Serialize,
{
    type Error: Error;

    fn as_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error>;
}
