use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use super::flat_collections::{FlatMap, FlatSet, FlatMapIter, FlatSetIter, FlatMapKeys};
use super::vec_collections::{VecMap, VecSet, VecMapIter, VecSetIter, VecMapKeys};
use super::id::EntityIdRaw;

pub type EntityHashMap<T> = HashMap<EntityIdRaw, T>;
pub type EntityHashSet = HashSet<EntityIdRaw>;
pub type EntityBTreeMap<T> = BTreeMap<EntityIdRaw, T>;
pub type EntityBTreeSet = BTreeSet<EntityIdRaw>;
pub type EntityFlatMap<T> = FlatMap<T>;
pub type EntityFlatSet = FlatSet;
pub type EntityVecMap<T> = VecMap<EntityIdRaw, T>;
pub type EntityVecSet = VecSet<EntityIdRaw>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityStoreRaw {
    {% for key, component in components %}
        {% if component.storage %}
            {% if component.type %}
                pub {{ key }}: {{ component.storage.rust_type }}<{{ component.type }}>,
            {% else %}
                pub {{ key }}: {{ component.storage.rust_type }},
            {% endif %}
        {% endif %}
    {% endfor %}
}

impl EntityStoreRaw {
    pub fn new() -> Self {
        Default::default()
    }
}
