#![allow(dead_code)]

use super::id::{EntityId, EntityIdRaw, EntityWit, EntityIdToFree, EntityIdToStore};
use super::entity_store_raw::*;
use super::iterators::*;
use super::component::*;
use super::spatial_hash::{SpatialHashTable, SpatialHashCell};
use super::entity_component_table::EntityComponentTable;
use super::component_type_set::*;
use entity_store_helper::grid_2d::{self, Size, Coord, CoordIter};
use entity_store_helper::IdAllocator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityStore {
    raw: EntityStoreRaw,
    spatial_hash: SpatialHashTable,
    id_allocator: IdAllocator<EntityIdRaw>,
    id_free_count: EntityFlatMap<u64>,
    entity_component_table: EntityComponentTable,
}

pub struct EntityIdIterOfRef<'a, 'w, I: Iterator<Item=&'a EntityIdRaw>> {
    iter: I,
    wit: &'w EntityWit<'w>,
}

impl<'a, 'w, I: Iterator<Item=&'a EntityIdRaw>> Iterator for EntityIdIterOfRef<'a, 'w, I> {
    type Item = EntityId<'w>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| {
            EntityId {
                raw: *raw,
                wit: *self.wit,
            }
        })
    }
}

impl<'a, 'w, I: Iterator<Item=&'a EntityIdRaw>> EntityIdIterOfRef<'a, 'w, I> {
    pub(super) fn new(iter: I, wit: &'w EntityWit<'w>) -> Self {
        EntityIdIterOfRef {
            iter,
            wit,
        }
    }
}

pub struct EntityIdAndValIterOfRef<'a, 'w, T: 'a, I: Iterator<Item=(&'a EntityIdRaw, &'a T)>> {
    iter: I,
    wit: &'w EntityWit<'w>,
}

impl<'a, 'w, T, I: Iterator<Item=(&'a EntityIdRaw, &'a T)>> Iterator for EntityIdAndValIterOfRef<'a, 'w, T, I> {
    type Item = (EntityId<'w>, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(raw, t)| {
            (EntityId {
                raw: *raw,
                wit: *self.wit,
            }, t)
        })
    }
}

impl<'a, 'w, T, I: Iterator<Item=(&'a EntityIdRaw, &'a T)>> EntityIdAndValIterOfRef<'a, 'w, T, I> {
    fn new(iter: I, wit: &'w EntityWit<'w>) -> Self {
        EntityIdAndValIterOfRef {
            iter,
            wit,
        }
    }
}

pub struct EntityIdIterOfVal<'w, I: Iterator<Item=EntityIdRaw>> {
    iter: I,
    wit: &'w EntityWit<'w>,
}

impl<'w, I: Iterator<Item=EntityIdRaw>> Iterator for EntityIdIterOfVal<'w, I> {
    type Item = EntityId<'w>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| {
            EntityId {
                raw,
                wit: *self.wit,
            }
        })
    }
}

impl<'w, I: Iterator<Item=EntityIdRaw>> EntityIdIterOfVal<'w, I> {
    fn new(iter: I, wit: &'w EntityWit<'w>) -> Self {
        Self {
            iter,
            wit,
        }
    }
}

pub struct EntityIdAndValIterOfVal<'a, 'w, T: 'a, I: Iterator<Item=(EntityIdRaw, &'a T)>> {
    iter: I,
    wit: &'w EntityWit<'w>
}

impl<'a, 'w, T, I: Iterator<Item=(EntityIdRaw, &'a T)>> Iterator for EntityIdAndValIterOfVal<'a, 'w, T, I> {
    type Item = (EntityId<'w>, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(raw, t)| {
            (EntityId {
                raw,
                wit: *self.wit,
            }, t)
        })
    }
}

impl<'a, 'w, T, I: Iterator<Item=(EntityIdRaw, &'a T)>> EntityIdAndValIterOfVal<'a, 'w, T, I> {
    fn new(iter: I, wit: &'w EntityWit<'w>) -> Self {
        EntityIdAndValIterOfVal {
            iter,
            wit,
        }
    }
}

pub struct ComponentDrain<'a> {
    entity_store: &'a mut EntityStore,
    entity_id: EntityIdRaw,
    component_type_set_iter: ComponentTypeSetIter,
}

impl<'a> Drop for ComponentDrain<'a> {
    fn drop(&mut self) {
        while let Some(component_type) = self.component_type_set_iter.next() {
            self.entity_store.raw_remove(self.entity_id, component_type);
        }
    }
}

impl<'a> Iterator for ComponentDrain<'a> {
    type Item = ComponentValue;
    fn next(&mut self) -> Option<Self::Item> {
        self.component_type_set_iter.next().and_then(|component_type| {
            self.entity_store.raw_remove(self.entity_id, component_type)
        })
    }
}

pub type SpatialHashIter<'a> = grid_2d::Iter<'a, SpatialHashCell>;
pub type SpatialHashCoordEnumerate<'a> = grid_2d::CoordEnumerate<'a, SpatialHashCell>;

impl EntityStore {
    pub fn new<'w>(size: Size) -> (Self, EntityWit<'w>) {
        (Self {
            raw: EntityStoreRaw::new(),
            spatial_hash: SpatialHashTable::new(size),
            id_allocator: IdAllocator::new(),
            id_free_count: EntityFlatMap::new(),
            entity_component_table: EntityComponentTable::new(),
        }, EntityWit::new())
    }

    pub fn entity_id_to_store<'a, 'w>(&'a self, id: EntityId<'w>) -> EntityIdToStore {
        let free_count = self.id_free_count.get(&id.raw).cloned().unwrap_or(0);
        EntityIdToStore {
            raw: id.raw,
            free_count,
        }
    }

    pub fn entity_id_from_stored<'a, 'w>(&'a self, wit: &'w EntityWit<'w>, id: EntityIdToStore) -> Option<EntityId<'w>> {
        let free_count = self.id_free_count.get(&id.raw).cloned().unwrap_or(0);
        if free_count == id.free_count {
            Some(EntityId {
                raw: id.raw,
                wit: *wit,
            })
        } else {
            None
        }
    }

    pub fn entity_id_to_free<'a, 'w>(&'a self, id: EntityId<'w>) -> EntityIdToFree {
        let free_count = self.id_free_count.get(&id.raw).cloned().unwrap_or(0);
        EntityIdToFree {
            raw: id.raw,
            free_count,
        }
    }

    pub fn drain_entity_components<'a, 'w>(&'a mut self, id: EntityId<'w>) -> ComponentDrain<'a> {
        let iter = if let Some(components) = self.entity_component_table.remove(id.raw) {
            components.iter()
        } else {
            ComponentTypeSetIter::empty()
        };
        ComponentDrain {
            entity_id: id.raw,
            component_type_set_iter: iter,
            entity_store: self,
        }
    }

    pub fn allocate_entity_id<'a, 'w>(&'a mut self, wit: &'w EntityWit) -> EntityId<'w> {
        let raw = self.id_allocator.allocate();
        EntityId {
            raw: raw,
            wit: *wit,
        }
    }

    pub fn remove_entity<'a, 'w>(&'a mut self, _wit: &'w mut EntityWit, id: EntityIdToFree) {
        let free_count = self.id_free_count.entry(&id.raw).or_insert(0);
        if id.free_count != *free_count {
            return;
        }
        self.id_allocator.free(id.raw);
        *free_count += 1;
        if let Some(components) = self.entity_component_table.remove(id.raw) {
            for component_type in components.iter() {
                match component_type {
                    {% for key, component in components %}
                        ComponentType::{{ component.name }} => {
                            self.raw.{{ key }}.remove(&id.raw);
                        }
                    {% endfor %}
                }
            }
        }
    }

    pub fn insert<'a, 'w>(&'a mut self, id: EntityId<'w>, value: ComponentValue) -> Option<ComponentValue> {
        match value {
            {% for key, component in components %}
                {% if component.storage %}
                    {% if component.type %}
                        ComponentValue::{{ component.name }}(value) => {
                            self.insert_{{ key }}(id, value).map(ComponentValue::{{ component.name }})
                        }
                    {% else %}
                        ComponentValue::{{ component.name }} => {
                            if self.insert_{{ key }}(id) {
                                Some(ComponentValue::{{ component.name }})
                            } else {
                                None
                            }
                        }
                    {% endif %}
                {% endif %}
            {% endfor %}
        }
    }

    pub fn remove<'a, 'w>(&'a mut self, id: EntityId<'w>, typ: ComponentType) -> Option<ComponentValue> {
        self.raw_remove(id.raw, typ)
    }
    fn raw_remove<'a>(&'a mut self, id: EntityIdRaw, typ: ComponentType) -> Option<ComponentValue> {
        match typ {
            {% for key, component in components %}
                {% if component.storage %}
                    {% if component.type %}
                        ComponentType::{{ component.name }} => {
                            self.raw_remove_{{ key }}(id).map(ComponentValue::{{ component.name }})
                        }
                    {% else %}
                        ComponentType::{{ component.name }} => {
                            if self.raw_remove_{{ key }}(id) {
                                Some(ComponentValue::{{ component.name }})
                            } else {
                                None
                            }
                        }
                    {% endif %}
                {% endif %}
            {% endfor %}
        }
    }

    pub fn spatial_hash_width(&self) -> u32 {
        self.spatial_hash.grid.width()
    }

    pub fn spatial_hash_height(&self) -> u32 {
        self.spatial_hash.grid.height()
    }

    pub fn spatial_hash_size(&self) -> Size {
        self.spatial_hash.grid.size()
    }

    pub fn spatial_hash_iter(&self) -> SpatialHashIter {
        self.spatial_hash.grid.iter()
    }

    pub fn spatial_hash_coords(&self) -> CoordIter {
        self.spatial_hash.grid.coords()
    }

    pub fn spatial_hash_enumerate(&self) -> SpatialHashCoordEnumerate {
        self.spatial_hash.grid.enumerate()
    }

    pub fn spatial_hash_get(&self, coord: Coord) -> Option<&SpatialHashCell> {
        self.spatial_hash.grid.get(coord.into())
    }

    {% for key, component in components %}
        {% if component.storage %}
            pub fn is_empty_{{ key }}(&self) -> bool {
                self.raw.{{ key }}.is_empty()
            }
            pub fn len_{{ key }}(&self) -> usize {
                self.raw.{{ key }}.len()
            }
            {% if component.type %}
                pub fn get_{{ key }}(&self, id: EntityId) -> Option<&{{ component.type }}> {
                    self.raw.{{ key }}.get(&id.raw)
                }
                pub fn contains_{{ key }}(&self, id: EntityId) -> bool {
                    self.raw.{{ key }}.contains_key(&id.raw)
                }
                pub fn iter_{{ key }}<'a, 'w>(&'a self, wit: &'w EntityWit<'w>) -> {{ component.storage.map_iter_wrapper }}<'a, 'w, {{ component.type }}, {{ component.storage.map_iter }}<{{ component.type }}>> {
                    {{ component.storage.map_iter_wrapper }}::new(self.raw.{{ key }}.iter(), wit)
                }
                pub fn ids_{{ key }}<'a, 'w>(&'a self, wit: &'w EntityWit<'w>) -> {{ component.storage.set_iter_wrapper }}<
                {% if component.storage.set_iter_wrapper != "EntityIdIterOfVal" %}
                    'a,
                {% endif %}
                'w, {{ component.storage.map_keys }}<{{ component.type }}>> {
                    {{ component.storage.set_iter_wrapper }}::new(self.raw.{{ key }}.keys(), wit)
                }
                pub fn any_{{ key }}<'w>(&self, wit: &'w EntityWit<'w>) -> Option<(EntityId<'w>, &{{ component.type }})> {
                    {% if component.storage.type == "vector" %}
                        self.raw.{{ key }}.first().map(|(&raw, value)| {
                            (EntityId {
                                raw,
                                wit: *wit,
                            }, value)
                        })
                    {% else %}
                        self.iter_{{ key }}(wit).next()
                    {% endif %}
                }
                pub fn any_id_{{ key }}<'w>(&self, wit: &'w EntityWit<'w>) -> Option<EntityId<'w>> {
                    {% if component.storage.type == "vector" %}
                        self.raw.{{ key }}.first_key().map(|&raw| {
                            EntityId {
                                raw,
                                wit: *wit,
                            }
                        })
                    {% else %}
                        self.ids_{{ key }}(wit).next()
                    {% endif %}
                }
                pub fn insert_{{ key }}(&mut self, id: EntityId, {{ key }}: {{ component.type }}) -> Option<{{ component.type }}> {
                    {% if component.tracked_by_spatial_hash %}
                        self.spatial_hash.raw_insert_{{ key }}(&self.raw, id.raw, &{{ key }});
                    {% endif %}
                    self.entity_component_table.insert_{{ key }}(id.raw);
                    self.raw.{{ key }}.insert(id.raw, {{ key }})
                }
                pub fn remove_{{ key }}(&mut self, id: EntityId) -> Option<{{ component.type }}> {
                    self.raw_remove_{{ key }}(id.raw)
                }
                fn raw_remove_{{ key }}(&mut self, id: EntityIdRaw) -> Option<{{ component.type }}> {
                    {% if component.tracked_by_spatial_hash %}
                        self.spatial_hash.raw_remove_{{ key }}(&self.raw, id);
                    {% endif %}
                    self.entity_component_table.remove_{{ key }}(id);
                    self.raw.{{ key }}.remove(&id)
                }
            {% else %}
                pub fn contains_{{ key }}(&self, id: EntityId) -> bool {
                    self.raw.{{ key }}.contains(&id.raw)
                }
                pub fn iter_{{ key }}<'a, 'w>(&'a self, wit: &'w EntityWit<'w>) -> {{ component.storage.set_iter_wrapper }}<
                {% if component.storage.set_iter_wrapper != "EntityIdIterOfVal" %}
                    'a,
                {% endif %}
                    'w, {{ component.storage.set_iter }}> {
                    {{ component.storage.set_iter_wrapper }}::new(self.raw.{{ key }}.iter(), wit)
                }
                pub fn any_{{ key }}<'w>(&self, wit: &'w EntityWit<'w>) -> Option<EntityId<'w>> {
                    {% if component.storage.type == "vector" %}
                        self.raw.{{ key }}.first().map(|&raw| {
                            EntityId {
                                raw,
                                wit: *wit,
                            }
                        })
                    {% else %}
                        self.iter_{{ key }}(wit).next()
                    {% endif %}
                }
                pub fn insert_{{ key }}(&mut self, id: EntityId) -> bool {
                    {% if component.tracked_by_spatial_hash %}
                        self.spatial_hash.raw_insert_{{ key }}(&self.raw, id.raw);
                    {% endif %}
                    self.entity_component_table.insert_{{ key }}(id.raw);
                    self.raw.{{ key }}.insert(id.raw)
                }
                pub fn remove_{{ key }}(&mut self, id: EntityId) -> bool {
                    self.raw_remove_{{ key }}(id.raw)
                }
                fn raw_remove_{{ key }}(&mut self, id: EntityIdRaw) -> bool {
                    {% if component.tracked_by_spatial_hash %}
                        self.spatial_hash.raw_remove_{{ key }}(&self.raw, id);
                    {% endif %}
                    self.entity_component_table.remove_{{ key }}(id);
                    self.raw.{{ key }}.remove(&id)
                }
            {% endif %}

        {% endif %}
    {% endfor %}
}
