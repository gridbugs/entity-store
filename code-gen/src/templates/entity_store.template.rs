#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use super::{EntityId, EntityVecMap, EntityVecSet, EntityChange, ComponentValue, ComponentType, insert};
use entity_store_helper::append::Append;

pub type EntityHashMap<T> = HashMap<EntityId, T>;
pub type EntityBTreeMap<T> = BTreeMap<EntityId, T>;
pub type EntityHashSet = HashSet<EntityId>;
pub type EntityBTreeSet = BTreeSet<EntityId>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityStore {
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

impl EntityStore {
    pub fn new() -> Self {
        Self {
            {% for key, component in components %}
                {% if component.storage %}
                    {{ key }}: {{ component.storage.rust_type }}::default(),
                {% endif %}
            {% endfor %}
        }
    }

    pub fn commit(&mut self, change: EntityChange) {
        match change {
            EntityChange::Insert(id, value) => match value {
                {% for key, component in components %}
                    {% if component.storage %}
                        {% if component.type %}
                            ComponentValue::{{ component.name }}(value) => { self.{{ key }}.insert(id, value); }
                        {% else %}
                            ComponentValue::{{ component.name }} => { self.{{ key }}.insert(id); }
                        {% endif %}
                    {% else %}
                        {% if component.type %}
                            ComponentValue::{{ component.name }}(_) => {}
                        {% else %}
                            ComponentValue::{{ component.name }} => {}
                        {% endif %}
                    {% endif %}
                {% endfor %}
            }
            EntityChange::Remove(id, typ) => match typ {
                {% for key, component in components %}
                    {% if component.storage %}
                        ComponentType::{{ component.name }} => { self.{{ key }}.remove(&id); }
                    {% else %}
                        ComponentType::{{ component.name }} => {}
                    {% endif %}
                {% endfor %}
            }
        }
    }

    pub fn clone_values<A: Append<(EntityId, ComponentValue)>>(&self, buf: &mut A) {
        {% for key, component in components %}
            {% if component.storage %}
                {% if component.type %}
                    for (id, value) in self.{{ key }}.iter() {
                        buf.append((id.clone(), ComponentValue::{{ component.name }}(value.clone())));
                    }
                {% else %}
                    for id in self.{{ key }}.iter() {
                        buf.append((id.clone(), ComponentValue::{{ component.name }}));
                    }
                {% endif %}
            {% endif %}
        {% endfor %}
    }

    pub fn clone_changes<A: Append<EntityChange>>(&self, buf: &mut A) {
        {% for key, component in components %}
            {% if component.storage %}
                {% if component.type %}
                    for (id, value) in self.{{ key }}.iter() {
                        buf.append(insert::{{ key }}(id.clone(), value.clone()));
                    }
                {% else %}
                    for id in self.{{ key }}.iter() {
                        buf.append(insert::{{ key }}(id.clone()));
                    }
                {% endif %}
            {% endif %}
        {% endfor %}
    }
}

impl Default for EntityStore {
    fn default() -> Self {
        Self::new()
    }
}
