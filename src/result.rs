use std::env;
use std::result;
use std::path::PathBuf;
use std::io;
use toml;
use tera;

/// Errors that can occur during code generation
#[derive(Debug)]
pub enum GenError {
    ParseError(toml::de::Error),
    SpecError,
    InvalidStorageType(Vec<String>),
    InvalidAggregateType(Vec<String>),
    NoSuchComponent(String),
    MissingSpatialHashKey,
    TemplateError(tera::Error),
    InvalidIdWidth(Vec<usize>),
    Utf8ConversionError,
    RustFmtError,
    MissingStorageType(String),
    NoComponents,
}

pub type GenResult<T> = result::Result<T, GenError>;

impl From<tera::Error> for GenError {
    fn from(e: tera::Error) -> Self {
        GenError::TemplateError(e)
    }
}

impl From<toml::de::Error> for GenError {
    fn from(e: toml::de::Error) -> Self {
        GenError::ParseError(e)
    }
}

/// Errors that can occur while creating the output file
#[derive(Debug)]
pub enum SaveError {
    VarError(env::VarError, &'static str),
    FailedToCreateFile(PathBuf, io::Error),
    FailedToWriteFile(PathBuf, io::Error),
    #[cfg(unix)]
    FailedToRemoveExistingSymlink(PathBuf, io::Error),
    #[cfg(unix)]
    FailedToMakeSymlink(PathBuf, io::Error),
}

pub type SaveResult<T> = result::Result<T, SaveError>;
