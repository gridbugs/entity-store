use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct StorageInfo {
    #[serde(rename = "type")]
    pub typ: String,
    pub rust_type: String,
}

#[derive(Debug, Serialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub name: String,
    pub storage: Option<StorageInfo>,
    pub index: usize,
    pub key: String,
}

#[derive(Debug, Serialize)]
pub struct Spec {
    pub components: BTreeMap<String, Component>,
    pub id_type: String,
    pub num_component_types: usize,
}
