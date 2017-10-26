use std::result;
use toml;

#[derive(Debug)]
pub enum Error {
    FailedToFormStringFromInputBytes,
    ParseError(toml::de::Error),
    SpecError,
    InvalidStorageType(Vec<String>),
    InvalidAggregateType(Vec<String>),
    NoSuchComponent(String),
    MissingSpatialHashKey,
}

pub type Result<T> = result::Result<T, Error>;
