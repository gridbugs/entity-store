use std::collections::BTreeMap;
use toml;
use result::{Result, Error};

fn ret_none<T>() -> Option<T> { None }

#[derive(Debug, Deserialize)]
pub struct Spec {
    #[serde(default = "BTreeMap::new")]
    pub components: BTreeMap<String, Component>,
    #[serde(default = "BTreeMap::new")]
    pub spatial_hash: BTreeMap<String, SpatialHashField>,
    #[serde(default = "ret_none")]
    pub spatial_hash_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Component {
    #[serde(rename = "type", default = "ret_none")]
    pub typ: Option<String>,
    #[serde(default = "ret_none")]
    pub name_override: Option<String>,
    #[serde(default = "ret_none")]
    pub storage: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SpatialHashField {
    pub component: String,
    #[serde(default = "ret_none")]
    pub aggregate: Option<String>,
}

impl Spec {
    pub fn from_str(s: &str) -> Result<Self> {
        toml::from_str(s).map_err(Error::ParseError)
    }
}
