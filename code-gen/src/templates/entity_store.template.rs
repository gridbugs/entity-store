#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use super::{EntityId, EntityVecMap, EntityVecSet, EntityChange, ComponentValue, ComponentRef,
            ComponentType, ComponentTypeSetIter, EntityComponentTable, insert};
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

    pub fn get(&self, id: EntityId, component_type: ComponentType) -> Option<ComponentRef> {
        match component_type {
            {% for key, component in components %}
                ComponentType::{{ component.name }} => {
                    {% if component.type %}
                        self.{{ key }}.get(&id).map(ComponentRef::{{ component.name }})
                    {% else %}
                        if self.{{ key }}.contains(&id) {
                            Some(ComponentRef::{{ component.name }})
                        } else {
                            None
                        }
                    {% endif %}
                }
            {% endfor %}
        }
    }

    pub fn contains(&self, id: EntityId, component_type: ComponentType) -> bool {
        match component_type {
            {% for key, component in components %}
                ComponentType::{{ component.name }} => {
                    {% if component.type %}
                        self.{{ key }}.contains_key(&id)
                    {% else %}
                        self.{{ key }}.contains(&id)
                    {% endif %}
                }
            {% endfor %}
        }
    }

    pub fn remove(&mut self, id: EntityId, component_type: ComponentType) -> Option<ComponentValue> {
        match component_type {
            {% for key, component in components %}
                ComponentType::{{ component.name }} => {
                    {% if component.type %}
                        self.{{ key }}.remove(&id).map(ComponentValue::{{ component.name }})
                    {% else %}
                        if self.{{ key }}.remove(&id) {
                            Some(ComponentValue::{{ component.name }})
                        } else {
                            None
                        }
                    {% endif %}
                }
            {% endfor %}
        }
    }

    pub fn insert(&mut self, id: EntityId, component_value: ComponentValue) -> Option<ComponentValue> {
        match component_value {
            {% for key, component in components %}
                {% if component.type %}
                    ComponentValue::{{ component.name }}(value) => {
                        self.{{ key }}.insert(id, value).map(ComponentValue::{{ component.name }})
                    }
                {% else %}
                    ComponentValue::{{ component.name }} => {
                        if self.{{ key }}.insert(id) {
                            Some(ComponentValue::{{ component.name }})
                        } else {
                            None
                        }
                    }
                {% endif %}
            {% endfor %}
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

    pub fn component_ref_iter(&self, entity_id: EntityId, component_type_iter: ComponentTypeSetIter) -> ComponentRefIter {
        ComponentRefIter {
            entity_store: self,
            entity_id,
            component_type_iter,
        }
    }

    pub fn component_drain(&mut self, entity_id: EntityId, component_type_iter: ComponentTypeSetIter) -> ComponentDrain {
        ComponentDrain {
            entity_store: self,
            entity_id,
            component_type_iter,
        }
    }

    pub fn component_drain_insert(&mut self, source_id: EntityId, dest_id: EntityId, component_type_iter: ComponentTypeSetIter)
        -> ComponentDrainInsert
    {
        let drain = self.component_drain(source_id, component_type_iter);
        ComponentDrainInsert {
            drain,
            dest_id,
        }
    }
}

impl Default for EntityStore {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ComponentRefIter<'a> {
    entity_store: &'a EntityStore,
    entity_id: EntityId,
    component_type_iter: ComponentTypeSetIter,
}

impl<'a> Iterator for ComponentRefIter<'a> {
    type Item = ComponentRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.component_type_iter.next().and_then(|component_type| {
            self.entity_store.get(self.entity_id, component_type)
        })
    }
}

pub struct ComponentDrain<'a> {
    entity_store: &'a mut EntityStore,
    entity_id: EntityId,
    component_type_iter: ComponentTypeSetIter,
}

impl<'a> Iterator for ComponentDrain<'a> {
    type Item = ComponentValue;
    fn next(&mut self) -> Option<Self::Item> {
        self.component_type_iter.next().and_then(|component_type| {
            self.entity_store.remove(self.entity_id, component_type)
        })
    }
}

pub struct ComponentDrainInsert<'a> {
    drain: ComponentDrain<'a>,
    dest_id: EntityId,
}

impl<'a> Iterator for ComponentDrainInsert<'a> {
    type Item = EntityChange;
    fn next(&mut self) -> Option<Self::Item> {
        self.drain.next().map(|value| {
            EntityChange::Insert(self.dest_id, value)
        })
    }
}
