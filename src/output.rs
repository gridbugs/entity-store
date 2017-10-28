use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct StorageInfo {
    #[serde(rename = "type")]
    pub typ: String,
    pub rust_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub name: String,
    pub storage: Option<StorageInfo>,
    pub index: usize,
    pub key: String,
    pub contains: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AggregateInfo {
    #[serde(rename = "type")]
    pub typ: String,
    pub rust_type: String,
}

#[derive(Debug, Serialize)]
pub struct ByComponentInfo {
    pub fields: BTreeMap<String, SpatialHashField>,
    pub lookup: Option<String>,
    pub component: Component,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpatialHashField {
    pub key: String,
    pub aggregate: Option<AggregateInfo>,
    pub component: Component,
}

#[derive(Debug, Serialize)]
pub struct SpatialHash {
    pub fields: BTreeMap<String, SpatialHashField>,
    pub by_component: BTreeMap<String, ByComponentInfo>,
    pub position_component: Component,
    pub has_neighbours: bool,
}

#[derive(Debug, Serialize)]
pub struct Spec {
    pub components: BTreeMap<String, Component>,
    pub spatial_hash: Option<SpatialHash>,
    pub id_type: String,
    pub num_component_types: usize,
}
