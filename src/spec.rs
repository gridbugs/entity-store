use std::collections::BTreeMap;
use itertools;
use storage_type::{self, StorageType};
use aggregate_type::{self, AggregateType};
use result::{Result, Error};
use input;

#[derive(Debug, Clone)]
pub struct Spec {
    components: ComponentSpec,
    spatial_hash: Option<SpatialHashSpec>,
}

#[derive(Debug, Clone)]
pub struct Component {
    typ: Option<String>,
    name: String,
    storage_type: Option<StorageType>,
}

#[derive(Debug, Clone)]
pub struct ComponentSpec {
    components: BTreeMap<String, Component>,
}

#[derive(Debug, Clone)]
pub struct SpatialHashField {
    aggregate_type: Option<AggregateType>,
    component_field: String,
}

#[derive(Debug, Clone)]
pub struct SpatialHashSpec {
    position_component: String,
    fields: BTreeMap<String, SpatialHashField>,
}

fn capitalise_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    }
}

fn name_from_field_name(field_name: &str) -> String {
    // convert underscore_case to CamelCase
    let with_first_capitalised = field_name.split('_').map(capitalise_first_letter);
    itertools::join(with_first_capitalised, "")
}

impl Spec {
    pub fn from_str(s: &str) -> Result<Self> {
        let spec_in = input::Spec::from_str(s)?;

        let components: Result<BTreeMap<String, Component>> =
            spec_in.components.iter().map(|(f, c_in)| {
                Component::from_input(f.as_str(), c_in).map(|c| {
                    (f.clone(), c)
                })
            }).collect();
        let components = components?;

        let spatial_hash_fields: Result<BTreeMap<String, SpatialHashField>> =
            spec_in.spatial_hash.iter().map(|(f, shf_in)| {
                SpatialHashField::from_input(&shf_in, &components).map(|shf| {
                    (f.clone(), shf)
                })
            }).collect();
        let spatial_hash_fields = spatial_hash_fields?;

        let spatial_hash = if let Some(shk) = spec_in.spatial_hash_key.as_ref() {
            if !components.contains_key(shk) {
                return Err(Error::NoSuchComponent(shk.clone()));
            }
            Some(SpatialHashSpec {
                position_component: shk.clone(),
                fields: spatial_hash_fields,
            })
        } else {
            if !spatial_hash_fields.is_empty() {
                return Err(Error::MissingSpatialHashKey);
            }
            None
        };

        let components = ComponentSpec {
            components,
        };

        Ok(Self {
            components,
            spatial_hash,
        })
    }
}

fn valid_storage_type_strings() -> Vec<String> {
    storage_type::ALL.iter().map(|s| s.to_str().to_string()).collect()
}
fn valid_aggregate_type_strings() -> Vec<String> {
    aggregate_type::ALL.iter().map(|s| s.to_str().to_string()).collect()
}

impl Component {
    fn from_input(field_name: &str, c: &input::Component) -> Result<Self> {
        let storage_type = if let Some(s) = c.storage.as_ref() {
            if let Some(s) = StorageType::from_str(s.as_str()) {
                Some(s)
            } else {
                return Err(Error::InvalidStorageType(
                        valid_storage_type_strings()));
            }
        } else {
            None
        };

        let name = c.name_override.as_ref().cloned().unwrap_or_else(|| {
            name_from_field_name(field_name)
        });

        Ok(Self {
            storage_type,
            name,
            typ: c.typ.clone(),
        })
    }
}

impl SpatialHashField {
    fn from_input(f: &input::SpatialHashField,
                  components: &BTreeMap<String, Component>) -> Result<Self> {
        let aggregate_type = if let Some(a) = f.aggregate.as_ref() {
            if let Some(a) = AggregateType::from_str(a.as_str()) {
                Some(a)
            } else {
                return Err(Error::InvalidAggregateType(
                        valid_aggregate_type_strings()));
            }
        } else {
            None
        };

        if !components.contains_key(&f.component) {
            return Err(Error::NoSuchComponent(f.component.clone()));
        }

        Ok(Self {
            aggregate_type,
            component_field: f.component.clone(),
        })
    }
}
