extern crate itertools;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod storage_type;
mod aggregate_type;
mod spec;
mod result;
mod input;

use spec::Spec;
use result::{Result, Error};

#[derive(Debug, Clone)]
pub struct EntityCodeGen {
    spec: Spec,
}

impl EntityCodeGen {
    pub fn new(component_toml: &[u8]) -> Result<Self> {
        let s = ::std::str::from_utf8(component_toml)
            .map_err(|_| Error::FailedToFormStringFromInputBytes)?;
        let spec = Spec::from_str(s)?;

        Ok(Self {
            spec,
        })
    }
}
