use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct StorageInfo {
    #[serde(rename = "type")]
    pub typ: String,
    pub rust_type: String,
    pub map_iter_wrapper: String,
    pub map_iter_mut_wrapper: String,
    pub set_iter_wrapper: String,
    pub set_iter: String,
    pub map_iter: String,
    pub map_iter_mut: String,
    pub map_keys: String,
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
    pub tracked_by_spatial_hash: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AggregateInfo {
    #[serde(rename = "type")]
    pub typ: String,
    pub rust_type: String,
}

#[derive(Debug, Serialize)]
pub struct ByComponentInfo {
    pub has_fields: bool,
    pub fields: BTreeMap<String, SpatialHashField>,
    pub lookup: Option<String>,
    pub component: Component,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpatialHashField {
    pub key: String,
    pub aggregate: AggregateInfo,
    pub component: Component,
}

#[derive(Debug, Serialize)]
pub struct SpatialHash {
    pub fields: BTreeMap<String, SpatialHashField>,
    pub by_component: BTreeMap<String, ByComponentInfo>,
    pub position_component: Component,
}

#[derive(Debug, Serialize)]
pub struct Spec {
    pub components: BTreeMap<String, Component>,
    pub spatial_hash: SpatialHash,
    pub id_type: String,
    pub num_component_types: usize,
}
