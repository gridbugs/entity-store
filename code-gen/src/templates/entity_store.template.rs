use std::marker::PhantomData;
use super::id::{EntityId, EntityIdRaw, EntityWit, EntityIdToFree};
use super::entity_store_raw::*;
use super::iterators::*;
use super::spatial_hash::{SpatialHashTable, SpatialHashCell};
use entity_store_helper::grid_2d::{self, Grid, Size, Coord, CoordIter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityStore {
    raw: EntityStoreRaw,
    spatial_hash: SpatialHashTable,
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

pub type SpatialHashIter<'a> = grid_2d::Iter<'a, SpatialHashCell>;
pub type SpatialHashCoordEnumerate<'a> = grid_2d::CoordEnumerate<'a, SpatialHashCell>;

impl EntityStore {
    pub fn new(size: Size) -> Self {
        Self {
            raw: EntityStoreRaw::new(),
            spatial_hash: SpatialHashTable::new(size),
        }
    }

    pub fn free_entity_id<'a, 'w>(&'a mut self, wit: &'w mut EntityWit, id: EntityIdToFree) {
        unimplemented!()
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
                    self.raw.{{ key }}.insert(id.raw, {{ key }})
                }
                pub fn remove_{{ key }}(&mut self, id: EntityId) -> Option<{{ component.type }}> {
                    {% if component.tracked_by_spatial_hash %}
                        self.spatial_hash.raw_remove_{{ key }}(&self.raw, id.raw);
                    {% endif %}
                    self.raw.{{ key }}.remove(&id.raw)
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
                    self.raw.{{ key }}.insert(id.raw)
                }
                pub fn remove_{{ key }}(&mut self, id: EntityId) -> bool {
                    {% if component.tracked_by_spatial_hash %}
                        self.spatial_hash.raw_remove_{{ key }}(&self.raw, id.raw);
                    {% endif %}
                    self.raw.{{ key }}.remove(&id.raw)
                }
            {% endif %}

        {% endif %}
    {% endfor %}
}
