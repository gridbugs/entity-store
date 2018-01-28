use std::collections::BTreeMap;
use toml;
use result::GenResult as Result;

fn ret_none<T>() -> Option<T> { None }
fn ret_64() -> usize { 64 }

#[derive(Debug, Deserialize)]
pub struct Spec {
    #[serde(default = "BTreeMap::new")]
    pub components: BTreeMap<String, Component>,
    #[serde(default = "BTreeMap::new")]
    pub spatial_hash: BTreeMap<String, SpatialHashField>,
    #[serde(default = "ret_none")]
    pub spatial_hash_key: Option<String>,
    #[serde(default = "ret_64")]
    pub id_width: usize,
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
    #[serde(rename = "type", default = "ret_none")]
    pub typ: Option<String>,
}

impl Spec {
    pub fn from_str(s: &str) -> Result<Self> {
        Ok(toml::from_str(s)?)
    }
}
