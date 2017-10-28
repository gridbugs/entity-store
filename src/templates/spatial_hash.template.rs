{% if spatial_hash %}
#![allow(unused_variables)]

use super::{EntityChange, EntityStore, EntityId, ComponentType, ComponentValue};
use entity_store_helpers::num::One;
use entity_store_helpers::direction::Directions;
use entity_store_helpers::cgmath::Vector2;

pub type UnsignedCoord = Vector2<u32>;
pub type SignedCoord = Vector2<i32>;

pub trait SpatialHashIndex {
    fn index(&self, width: usize) -> Option<usize>;
}

impl SpatialHashIndex for UnsignedCoord {
    fn index(&self, width: usize) -> Option<usize> {
        if self.x as usize >= width {
            return None;
        }

        Some(self.y as usize * width + self.x as usize)
    }
}

impl SpatialHashIndex for SignedCoord {
    fn index(&self, width: usize) -> Option<usize> {
        if self.x < 0 || self.y < 0 || self.x as usize >= width {
            return None;
        }

        Some(self.y as usize * width + self.x as usize)
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
                        self.{{ field.key }} += One::one();
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
                        self.{{ field.key }} -= One::one();
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
    width_usize: usize,
    width_i32: i32,
    height_i32: i32,
    height_u32: u32,
    width_u32: u32,
    cells: Vec<SpatialHashCell>,
}

impl SpatialHashTable {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut cells = Vec::new();
        cells.resize(size, Default::default());

        Self {
            width_usize: width as usize,
            width_i32: width as i32,
            height_i32: height as i32,
            width_u32: width,
            height_u32: height,
            cells,
        }
    }

    pub fn width(&self) -> u32 { self.width_u32 }
    pub fn height(&self) -> u32 { self.height_u32 }

    pub fn get<T: SpatialHashIndex>(&self, index: T) -> Option<&SpatialHashCell> {
        index.index(self.width_usize).and_then(|i| self.cells.get(i))
    }

    pub fn get_mut<T: SpatialHashIndex>(&mut self, index: T) -> Option<&mut SpatialHashCell> {
        if let Some(i) = index.index(self.width_usize) {
            return self.cells.get_mut(i);
        }
        None
    }

    pub fn update(&mut self, entity_store: &EntityStore, change: &EntityChange, time: u64) {
        match change {
            &EntityChange::Insert(id, ref value) => {
                match value {
                    &ComponentValue::{{ spatial_hash.position_component.name }}(position) => {
                        if let Some(current) = entity_store.{{ spatial_hash.position_component.key }}.get(&id) {
                            if let Some(cell) = self.get_mut(*current) {
                                cell.remove(id, entity_store, time);
                            }
                            self.remove_neighbours(id, entity_store, time, (*current).into());
                        }
                        if let Some(cell) = self.get_mut(position) {
                            cell.insert(id, entity_store, time);
                        }
                        self.insert_neighbours(id, entity_store, time, position.into());
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
                                            let normalized: SignedCoord = (*position).into();
                                            for d in Directions {
                                                if let Some(cell) = self.get_mut(normalized + d.vector()) {
                                                    cell.{{ field.key }}.inc(d.opposite());
                                                    cell.last_updated = time;
                                                }
                                            }
                                        }
                                    {% endif %}
                                {% endfor %}

                                {% if by_component.lookup %}
                                    if let Some(cell) = self.get_mut(*position) {
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
                                                cell.{{ field.key }} += One::one();
                                            {% elif field.aggregate.type == "set" %}
                                                cell.{{ field.key }}.insert(id);
                                            {% endif %}
                                        {% endfor %}
                                            cell.last_updated = time;
                                        }
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
                            if let Some(cell) = self.get_mut(*current) {
                                cell.remove(id, entity_store, time);
                            }
                            {% if spatial_hash.has_neighbours %}
                            self.remove_neighbours(id, entity_store, time, (*current).into());
                            {% endif %}
                        }
                    }
                    {% for _, by_component in spatial_hash.by_component %}
                        ComponentType::{{ by_component.component.name }} => {
                            if let Some(position) = entity_store.{{ spatial_hash.position_component.key }}.get(&id) {
                                {% for _, field in by_component.fields %}
                                    {% if field.aggregate.type == "neighbour_count" %}
                                        if entity_store.{{ by_component.component.key }}.{{ by_component.component.contains }}(&id) {
                                            let normalized: SignedCoord = (*position).into();
                                            for d in Directions {
                                                if let Some(cell) = self.get_mut(normalized + d.vector()) {
                                                    cell.{{ field.key }}.dec(d.opposite());
                                                    cell.last_updated = time;
                                                }
                                            }
                                        }
                                    {% endif %}
                                {% endfor %}

                                {% if by_component.lookup %}
                                    if let Some(cell) = self.get_mut(*position) {
                                        {% if by_component.lookup == "get" %}
                                            if let Some(current) = entity_store.{{ by_component.component.key }}.get(&id) {
                                        {% else %}
                                            if entity_store.{{ by_component.component.key }}.{{ by_component.component.contains }}(&id) {
                                        {% endif %}

                                        {% for _, field in by_component.fields %}
                                            {% if field.aggregate.type == "total" %}
                                                cell.{{ field.key }} -= *current;
                                            {% elif field.aggregate.type == "count" %}
                                                cell.{{ field.key }} -= One::one();
                                            {% elif field.aggregate.type == "set" %}
                                                cell.{{ field.key }}.remove(&id);
                                            {% endif %}
                                        {% endfor %}
                                            cell.last_updated = time;
                                        }
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
        fn insert_neighbours(&mut self, id: EntityId, entity_store: &EntityStore, time: u64, coord: SignedCoord) {
            {% for _, field in spatial_hash.fields %}
                {% if field.aggregate.type == "neighbour_count" %}
                    if entity_store.{{ field.component.key }}.{{ field.component.contains }}(&id) {
                        for d in Directions {
                            if let Some(cell) = self.get_mut(coord + d.vector()) {
                                cell.{{ field.key }}.inc(d.opposite());
                                cell.last_updated = time;
                            }
                        }
                    }
                {% endif %}
            {% endfor %}
        }

        fn remove_neighbours(&mut self, id: EntityId, entity_store: &EntityStore, time: u64, coord: SignedCoord) {
            {% for _, field in spatial_hash.fields %}
                {% if field.aggregate.type == "neighbour_count" %}
                    if entity_store.{{ field.component.key }}.{{ field.component.contains }}(&id) {
                        for d in Directions {
                            if let Some(cell) = self.get_mut(coord + d.vector()) {
                                cell.{{ field.key }}.dec(d.opposite());
                                cell.last_updated = time;
                            }
                        }
                    }
                {% endif %}
            {% endfor %}
        }
    {% endif %}
}
{% endif %}
