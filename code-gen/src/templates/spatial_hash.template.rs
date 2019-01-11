{% if spatial_hash %}
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_patterns)]

use super::{EntityChange, EntityStore, EntityId, ComponentType, ComponentValue};
use entity_store_helper::num::One;
use entity_store_helper::direction::Directions;

use entity_store_helper::grid_2d;
pub use entity_store_helper::grid_2d::{Grid, Size, Coord};
pub use entity_store_helper::grid_2d::coord_system::XThenYIter as CoordIter;

pub type Iter<'a> = grid_2d::GridIter<'a, SpatialHashCell>;
pub type CoordEnumerate<'a> = grid_2d::GridEnumerate<'a, SpatialHashCell>;

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
    fn insert(&mut self, id: EntityId, entity_store: &EntityStore, time: u64) {
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
        self.last_updated = time;
    }
    fn remove(&mut self, id: EntityId, entity_store: &EntityStore, time: u64) {
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
        self.last_updated = time;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialHashTable {
    grid: Grid<SpatialHashCell>,
}

impl SpatialHashTable {
    pub fn new(size: Size) -> Self {
        Self {
            grid: Grid::new_default(size),
        }
    }

    pub fn width(&self) -> u32 {
        self.grid.width()
    }

    pub fn height(&self) -> u32 {
        self.grid.height()
    }

    pub fn size(&self) -> Size {
        self.grid.size()
    }

    pub fn iter(&self) -> Iter {
        self.grid.iter()
    }

    pub fn coords(&self) -> CoordIter {
        self.grid.coord_iter()
    }

    pub fn enumerate(&self) -> CoordEnumerate {
        self.grid.enumerate()
    }

    pub fn get<T: Into<Coord>>(&self, coord: T) -> Option<&SpatialHashCell> {
        self.grid.get(coord.into())
    }

    fn get_mut<T: Into<Coord>>(&mut self, coord: T) -> Option<&mut SpatialHashCell> {
        self.grid.get_mut(coord.into())
    }

    pub fn update(&mut self, entity_store: &EntityStore, change: &EntityChange, time: u64) {
        match change {
            &EntityChange::Insert(id, ref value) => {
                match value {
                    &ComponentValue::{{ spatial_hash.position_component.name }}(position) => {
                        if let Some(current) = entity_store.{{ spatial_hash.position_component.key }}.get(&id) {
                            if let Some(cell) = self.grid.get_mut(*current) {
                                cell.remove(id, entity_store, time);
                            }
                            {% if spatial_hash.has_neighbours %}
                            self.remove_neighbours(id, entity_store, time, *current);
                            {% endif %}
                        }
                        if let Some(cell) = self.grid.get_mut(position) {
                            cell.insert(id, entity_store, time);
                        }
                        {% if spatial_hash.has_neighbours %}
                        self.insert_neighbours(id, entity_store, time, position);
                        {% endif %}
                    }
                    {% for _, by_component in spatial_hash.by_component %}
                        {% if by_component.component.type %}
                            &ComponentValue::{{ by_component.component.name }}(value) => {
                        {% else %}
                            &ComponentValue::{{ by_component.component.name }} => {
                        {% endif %}
                            if let Some(position) = entity_store.{{ spatial_hash.position_component.key }}.get(&id) {
                                {% for _, field in by_component.fields %}
                                    {% if field.aggregate.type == "neighbour_count" %}
                                        if !entity_store.{{ by_component.component.key }}.{{ by_component.component.contains }}(&id) {
                                            for d in Directions {
                                                if let Some(cell) = self.grid.get_mut(*position + d.coord()) {
                                                    cell.{{ field.key }}.inc(d.opposite());
                                                }
                                            }
                                        }
                                        cell.last_updated = time;
                                    {% endif %}
                                {% endfor %}

                                {% if by_component.lookup %}
                                    if let Some(cell) = self.grid.get_mut(*position) {
                                        {% if by_component.lookup == "get" %}
                                            if let Some(current) = entity_store.{{ by_component.component.key }}.get(&id) {
                                                {% for _, field in by_component.fields %}
                                                    {% if field.aggregate.type == "total" %}
                                                        let increase = value - *current;
                                                        cell.{{ field.key }} += increase;
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
                                        cell.last_updated = time;
                                    }
                                {% endif %}
                            }
                        }
                    {% endfor %}
                    _ => {}
                }
            }
            &EntityChange::Remove(id, typ) => {
                match typ {
                    ComponentType::{{ spatial_hash.position_component.name }} => {
                        if let Some(current) = entity_store.{{ spatial_hash.position_component.key }}.get(&id) {
                            if let Some(cell) = self.grid.get_mut(*current) {
                                cell.remove(id, entity_store, time);
                            }
                            {% if spatial_hash.has_neighbours %}
                            self.remove_neighbours(id, entity_store, time, *current);
                            {% endif %}
                        }
                    }
                    {% for _, by_component in spatial_hash.by_component %}
                        ComponentType::{{ by_component.component.name }} => {
                            if let Some(position) = entity_store.{{ spatial_hash.position_component.key }}.get(&id) {
                                {% for _, field in by_component.fields %}
                                    {% if field.aggregate.type == "neighbour_count" %}
                                        if entity_store.{{ by_component.component.key }}.{{ by_component.component.contains }}(&id) {
                                            for d in Directions {
                                                if let Some(cell) = self.grid.get_mut(*position + d.vector()) {
                                                    cell.{{ field.key }}.dec(d.opposite());
                                                }
                                            }
                                        }
                                        cell.last_updated = time;
                                    {% endif %}
                                {% endfor %}

                                {% if by_component.lookup %}
                                    if let Some(cell) = self.grid.get_mut(*position) {
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
                                        cell.last_updated = time;
                                    }
                                {% endif %}
                            }
                        }
                    {% endfor %}
                    _ => {}
                }
            }
        }
    }

    {% if spatial_hash.has_neighbours %}
        fn insert_neighbours(&mut self, id: EntityId, entity_store: &EntityStore, time: u64, coord: Coord) {
            {% for _, field in spatial_hash.fields %}
                {% if field.aggregate.type == "neighbour_count" %}
                    if entity_store.{{ field.component.key }}.{{ field.component.contains }}(&id) {
                        for d in Directions {
                            if let Some(cell) = self.grid.get_mut(coord + d.coord()) {
                                cell.{{ field.key }}.inc(d.opposite());
                            }
                        }
                    }
                    cell.last_updated = time;
                {% endif %}
            {% endfor %}
        }

        fn remove_neighbours(&mut self, id: EntityId, entity_store: &EntityStore, time: u64, coord: Coord) {
            {% for _, field in spatial_hash.fields %}
                {% if field.aggregate.type == "neighbour_count" %}
                    if entity_store.{{ field.component.key }}.{{ field.component.contains }}(&id) {
                        for d in Directions {
                            if let Some(cell) = self.grid.get_mut(coord + d.coord()) {
                                cell.{{ field.key }}.dec(d.opposite());
                            }
                        }
                    }
                    cell.last_updated = time;
                {% endif %}
            {% endfor %}
        }
    {% endif %}
}
{% endif %}
