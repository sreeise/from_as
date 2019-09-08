use std::ffi::OsStr;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum FromToError {
    Io(std::io::Error),
    SerdeJsonError(serde_json::error::Error),
    SerdeYamlError(serde_yaml::Error),
}

impl FromToError {
    pub fn invalid_extension(ext: &OsStr) -> FromToError {
        let s = format!("Expected json or yaml extension, got: {:#?}", ext);
        FromToError::Io(std::io::Error::new(ErrorKind::InvalidData, s))
    }
}

impl std::error::Error for FromToError {
    fn description(&self) -> &str {
        match *self {
            FromToError::Io(ref err) => err.description(),
            FromToError::SerdeJsonError(ref err) => err.description(),
            FromToError::SerdeYamlError(ref err) => err.description(),
        }
    }

    fn source<'a>(&'a self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FromToError::Io(ref err) => Some(err),
            FromToError::SerdeJsonError(ref err) => Some(err),
            FromToError::SerdeYamlError(ref err) => Some(err),
        }
    }
}

impl std::fmt::Display for FromToError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nError Code: {:#?}", &self)
    }
}

impl From<std::io::Error> for FromToError {
    fn from(err: std::io::Error) -> Self {
        FromToError::Io(err)
    }
}

impl From<serde_json::error::Error> for FromToError {
    fn from(err: serde_json::error::Error) -> Self {
        FromToError::SerdeJsonError(err)
    }
}

impl From<serde_yaml::Error> for FromToError {
    fn from(err: serde_yaml::Error) -> Self {
        FromToError::SerdeYamlError(err)
    }
}
