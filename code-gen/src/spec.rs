use std::collections::{BTreeMap, BTreeSet};
use itertools;
use storage_type::{self, StorageType};
use aggregate_type::{self, AggregateType};
use result::GenResult as Result;
use result::GenError as Error;
use input;
use output;

#[derive(Debug, Clone)]
pub struct Spec {
    components: ComponentSpec,
    spatial_hash: SpatialHashSpec,
}

#[derive(Debug, Clone)]
pub struct Component {
    typ: Option<String>,
    name: String,
    key: String,
    storage_type: Option<StorageType>,
}

#[derive(Debug, Clone)]
pub struct ComponentSpec {
    id_width: usize,
    components: BTreeMap<String, Component>,
}

#[derive(Debug, Clone)]
pub struct SpatialHashField {
    typ: Option<String>,
    aggregate_type: Option<AggregateType>,
    component: Component,
}

#[derive(Debug, Clone)]
pub struct SpatialHashSpec {
    position_component: String,
    fields: BTreeMap<String, SpatialHashField>,
    tracked_components: BTreeSet<String>,
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
            key: field_name.to_string(),
            typ: c.typ.clone(),
        })
    }

    fn to_output(&self, key: &str, index: usize, spatial_hash: &SpatialHashSpec) -> output::Component {
        let storage = self.storage_type.as_ref().map(|s| {
            output::StorageInfo {
                typ: s.to_str().to_string(),
                rust_type: {
                    if self.typ.is_some() {
                        s.to_map_type().to_string()
                    } else {
                        s.to_set_type().to_string()
                    }
                },
                map_iter_wrapper: s.to_map_iter_wrapper().to_string(),
                set_iter_wrapper: s.to_set_iter_wrapper().to_string(),
                map_iter: s.to_map_iter().to_string(),
                map_keys: s.to_map_keys().to_string(),
                set_iter: s.to_set_iter().to_string(),
            }
        });
        let tracked_by_spatial_hash = spatial_hash.tracked_components.contains(key) ||
            spatial_hash.position_component == key;
        output::Component {
            typ: self.typ.clone(),
            name: self.name.clone(),
            storage,
            key: key.to_string(),
            index,
            contains: if self.typ.is_some() { "contains_key".to_string() } else { "contains".to_string() },
            tracked_by_spatial_hash,
        }
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

        let component = if let Some(c) = components.get(&f.component) {
            c.clone()
        } else {
            return Err(Error::NoSuchComponent(f.component.clone()));
        };

        if let Some(a) = aggregate_type {
            if a.requires_storage() && component.storage_type.is_none() {
                return Err(Error::MissingStorageType(f.component.clone()));
            }
        }

        Ok(Self {
            aggregate_type,
            component,
            typ: f.typ.clone(),
        })
    }

    fn to_output(&self, key: &str, components: &BTreeMap<String, output::Component>) -> output::SpatialHashField {
        let aggregate = self.aggregate_type.map(|a| {
            output::AggregateInfo {
                typ: a.to_str().to_string(),
                rust_type: a.to_type(self.component.typ.as_ref(), self.typ.as_ref()),
            }
        }).unwrap_or_else(|| {
            output::AggregateInfo {
                typ: "void".to_string(),
                rust_type: "()".to_string(),
            }
        });
        let component = components.get(&self.component.key).unwrap().clone();
        output::SpatialHashField {
            key: key.to_string(),
            aggregate,
            component,
        }
    }
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

        if components.is_empty() {
            return Err(Error::NoComponents);
        }

        let spatial_hash_fields: Result<BTreeMap<String, SpatialHashField>> =
            spec_in.spatial_hash.iter().map(|(f, shf_in)| {
                SpatialHashField::from_input(&shf_in, &components).map(|shf| {
                    (f.clone(), shf)
                })
            }).collect();
        let spatial_hash_fields = spatial_hash_fields?;

        let tracked_components = spatial_hash_fields.values().map(|v| v.component.key.clone()).collect::<BTreeSet<_>>();

        let spatial_hash = if let Some(shk) = spec_in.spatial_hash_key.as_ref() {
            if !components.contains_key(shk) {
                return Err(Error::NoSuchComponent(shk.clone()));
            }
            SpatialHashSpec {
                position_component: shk.clone(),
                fields: spatial_hash_fields,
                tracked_components,
            }
        } else {
            return Err(Error::MissingSpatialHashKey);
        };

        let valid_id_widths = &[8, 16, 32, 64];
        if !valid_id_widths.contains(&spec_in.id_width) {
            return Err(Error::InvalidIdWidth(valid_id_widths.iter().cloned().collect()));
        }

        let components = ComponentSpec {
            components,
            id_width: spec_in.id_width,
        };

        Ok(Self {
            components,
            spatial_hash,
        })
    }

    pub fn to_output(&self) -> output::Spec {
        let components: BTreeMap<String, output::Component> = self.components.components.iter()
            .enumerate()
            .map(|(i, (k, v))| (k.clone(), v.to_output(k, i, &self.spatial_hash)) ).collect();

        let spatial_hash = {
            let fields: BTreeMap<String, output::SpatialHashField> = self.spatial_hash.fields.iter()
                .map(|(k, f)| (k.clone(), f.to_output(k, &components))).collect();
            let position_component = components.get(&self.spatial_hash.position_component).cloned().unwrap();
            let mut by_component = BTreeMap::new();
            for (f, g) in izip!(fields.values(), self.spatial_hash.fields.values()) {
                let current = by_component.entry(f.component.key.clone())
                    .or_insert_with(|| output::ByComponentInfo {
                        has_fields: false,
                        fields: BTreeMap::new(),
                        lookup: None,
                        component: f.component.clone(),
                    });

                if let Some(a) = g.aggregate_type {
                    if let Some(l) = a.to_lookup() {
                        current.lookup = match l {
                            "get" => Some("get"),
                            "contains" => {
                                if let Some(ref l) = current.lookup {
                                    if l.as_str() == "get" {
                                        Some("get")
                                    } else {
                                        Some("contains")
                                    }
                                } else {
                                    Some("contains")
                                }
                            }
                            _ => unreachable!(),
                        }.map(|s| s.to_string());
                    }
                    current.has_fields = true;
                }

                current.fields.insert(f.key.clone(), f.clone());
            }
            output::SpatialHash {
                fields,
                by_component,
                position_component,
            }
        };

        output::Spec {
            num_component_types: self.components.components.len(),
            components,
            id_type: format!("u{}", self.components.id_width),
            spatial_hash,
        }
    }
}
