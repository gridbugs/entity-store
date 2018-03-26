#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use super::id::{EntityIdRaw, EntityWit, EntityId};
use super::entity_store_raw::EntityStoreRaw;
use entity_store_helper::grid_2d::{Grid, Size};
use entity_store_helper::num::One;
use super::entity_store_raw::EntityVecSet;
use super::entity_store::EntityIdIterOfRef;
use super::iterators::EntityVecSetIter;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpatialHashCellEntityIdSet {
    set: EntityVecSet,
}

impl SpatialHashCellEntityIdSet {
    fn insert(&mut self, id: EntityIdRaw) {
        self.set.insert(id);
    }
    fn remove(&mut self, id: &EntityIdRaw) {
        self.set.remove(id);
    }
    pub fn iter<'a, 'w>(&'a self, wit: &'w EntityWit) -> EntityIdIterOfRef<'a, 'w, EntityVecSetIter<'a>> {
        EntityIdIterOfRef::new(self.set.iter(), wit)
    }
    pub fn any<'a, 'w>(&'a self, wit: &'w EntityWit) -> Option<EntityId<'w>> {
        self.set.first().map(|&raw| {
            EntityId {
                raw,
                wit: wit,
            }
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpatialHashCell {
    {% for key, field in spatial_hash.fields %}
        {% if field.aggregate %}
            pub {{ key }}: {{ field.aggregate.rust_type }},
        {% endif %}
    {% endfor %}
    pub last_updated: u64,
}

impl SpatialHashCell {
    fn raw_insert(&mut self, entity_store: &EntityStoreRaw, last_updated: u64, id: EntityIdRaw) {
        {% for _, by_component in spatial_hash.by_component %}
            {% if by_component.lookup %}
                {% if by_component.lookup == "get" %}
                    if let Some(current) = entity_store.{{ by_component.component.key }}.get(&id) {
                {% else %}
                    if entity_store.{{ by_component.component.key }}.{{ by_component.component.contains }}(&id) {
                {% endif %}

                {% for _, field in by_component.fields %}
                    {% if field.aggregate.type == "total" %}
                        self.{{ field.key }} += *current;
                    {% elif field.aggregate.type == "count" %}
                        self.{{ field.key }} += {{ field.aggregate.rust_type }}::one();
                    {% elif field.aggregate.type == "set" %}
                        self.{{ field.key }}.insert(id);
                    {% endif %}
                {% endfor %}

                }
            {% endif %}
        {% endfor %}
        self.last_updated = last_updated;
    }
    fn raw_remove(&mut self, entity_store: &EntityStoreRaw, last_updated: u64, id: EntityIdRaw) {
        {% for _, by_component in spatial_hash.by_component %}
            {% if by_component.lookup %}
                {% if by_component.lookup == "get" %}
                    if let Some(current) = entity_store.{{ by_component.component.key }}.get(&id) {
                {% else %}
                    if entity_store.{{ by_component.component.key }}.{{ by_component.component.contains }}(&id) {
                {% endif %}

                {% for _, field in by_component.fields %}
                    {% if field.aggregate.type == "total" %}
                        self.{{ field.key }} -= *current;
                    {% elif field.aggregate.type == "count" %}
                        self.{{ field.key }} -= {{ field.aggregate.rust_type }}::one();
                    {% elif field.aggregate.type == "set" %}
                        self.{{ field.key }}.remove(&id);
                    {% endif %}
                {% endfor %}

                }
            {% endif %}
        {% endfor %}
        self.last_updated = last_updated;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialHashTable {
    pub(super) grid: Grid<SpatialHashCell>,
    last_updated: u64,
}

impl SpatialHashTable {
    pub fn new(size: Size) -> Self {
        Self {
            grid: Grid::new_default(size),
            last_updated: 0,
        }
    }

    pub fn raw_insert_{{ spatial_hash.position_component.key }}(&mut self, entity_store: &EntityStoreRaw, id: EntityIdRaw, coord: &{{ spatial_hash.position_component.type }}) {
        if let Some(current) = entity_store.{{ spatial_hash.position_component.key }}.get(&id) {
            if let Some(cell) = self.grid.get_mut(*current) {
                cell.raw_remove(entity_store, self.last_updated, id);
            }
        }
        if let Some(cell) = self.grid.get_mut(*coord) {
            cell.raw_insert(entity_store, self.last_updated, id);
        }
        self.last_updated += 1;
    }

    pub fn raw_remove_{{ spatial_hash.position_component.key }}(&mut self, entity_store: &EntityStoreRaw, id: EntityIdRaw) {
        if let Some(current) = entity_store.{{ spatial_hash.position_component.key }}.get(&id) {
            if let Some(cell) = self.grid.get_mut(*current) {
                cell.raw_remove(entity_store, self.last_updated, id);
                self.last_updated += 1;
            }
        }
    }

    {% for _, by_component in spatial_hash.by_component %}
        {% if by_component.component.type %}
        pub fn raw_insert_{{ by_component.component.key }}(
            &mut self,
            entity_store: &EntityStoreRaw,
            id: EntityIdRaw,
            {% if by_component.has_fields %}value{% else %}_value{% endif %}: &{{ by_component.component.type}}) {
        {% else %}
        pub fn raw_insert_{{ by_component.component.key }}(&mut self, entity_store: &EntityStoreRaw, id: EntityIdRaw) {
        {% endif %}
            if let Some(coord) = entity_store.{{ spatial_hash.position_component.key }}.get(&id) {
                if let Some(cell) = self.grid.get_mut(*coord) {
                    {% if by_component.has_fields %}
                        {% if by_component.lookup == "get" %}
                            if let Some(current) = entity_store.{{ by_component.component.key }}.get(&id) {
                                {% for _, field in by_component.fields %}
                                    {% if field.aggregate.type == "total" %}

                                        if value > current {
                                            let increase = value - *current;
                                            cell.{{ field.key }} += increase;
                                        } else {
                                            let decrease = *current - value;
                                            cell.{{ field.key }} -= decrease;
                                        }

                                    {% endif %}
                                {% endfor %}
                            } else {
                        {% else %}
                            if !entity_store.{{ by_component.component.key }}.{{ by_component.component.contains }}(&id) {
                        {% endif %}

                            {% for _, field in by_component.fields %}
                                {% if field.aggregate.type == "total" %}
                                    cell.{{ field.key }} += value;
                                {% elif field.aggregate.type == "count" %}
                                    cell.{{ field.key }} += {{ field.aggregate.rust_type }}::one();
                                {% elif field.aggregate.type == "set" %}
                                    cell.{{ field.key }}.insert(id);
                                {% endif %}
                            {% endfor %}
                        }
                    {% endif %}

                    cell.last_updated = self.last_updated;
                    self.last_updated += 1;
                }
            }
        }
        pub fn raw_remove_{{ by_component.component.key }}(&mut self, entity_store: &EntityStoreRaw, id: EntityIdRaw) {
            if let Some(coord) = entity_store.{{ spatial_hash.position_component.key }}.get(&id) {
                if let Some(cell) = self.grid.get_mut(*coord) {


                    {% if by_component.lookup %}
                        {% if by_component.lookup == "get" %}
                            if let Some(current) = entity_store.{{ by_component.component.key }}.get(&id) {
                        {% else %}
                            if entity_store.{{ by_component.component.key }}.{{ by_component.component.contains }}(&id) {
                        {% endif %}

                        {% for _, field in by_component.fields %}
                            {% if field.aggregate.type == "total" %}
                                cell.{{ field.key }} -= *current;
                            {% elif field.aggregate.type == "count" %}
                                cell.{{ field.key }} -= {{ field.aggregate.rust_type }}::one();
                            {% elif field.aggregate.type == "set" %}
                                cell.{{ field.key }}.remove(&id);
                            {% endif %}
                        {% endfor %}
                        }
                    {% endif %}

                    cell.last_updated = self.last_updated;
                    self.last_updated += 1;
                }
            }
        }
    {% endfor %}
}
