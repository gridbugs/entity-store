#![allow(dead_code)]

use super::{EntityVecMap, ComponentTypeSet, ComponentTypeSetIter, EntityChange,
            EntityId, EntityStore, ComponentRefIter, ComponentDrain, ComponentDrainInsert};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentTable(EntityVecMap<ComponentTypeSet>);

impl EntityComponentTable {
    pub fn new() -> Self {
        EntityComponentTable(EntityVecMap::new())
    }
    pub fn update(&mut self, change: &EntityChange) {
        match change {
            &EntityChange::Insert(id, ref value) => {
                self.0.entry(&id).or_insert_with(|| ComponentTypeSet::new()).insert(value.typ());
            }
            &EntityChange::Remove(id, typ) => {
                if let Some(set) = self.0.get_mut(&id) {
                    set.remove(typ);
                }
            }
        }
    }

    pub fn get(&self, id: EntityId) -> ComponentTypeSet {
        self.0.get(&id).cloned().unwrap_or_else(ComponentTypeSet::new)
    }

    pub fn component_types(&self, id: EntityId) -> ComponentTypeSetIter {
        self.0.get(&id)
            .map(|s| s.iter())
            .unwrap_or_else(|| ComponentTypeSetIter::empty())
    }

    pub fn remove_entity(&self, id: EntityId) -> RemoveEntityIter {
        RemoveEntityIter {
            id,
            iter: self.component_types(id),
        }
    }

    pub fn component_ref_iter<'a>(&self, id: EntityId, entity_store: &'a EntityStore) -> ComponentRefIter<'a> {
        entity_store.component_ref_iter(id, self.component_types(id))
    }

    pub fn component_drain<'a>(&self, id: EntityId, entity_store: &'a mut EntityStore) -> ComponentDrain<'a> {
        entity_store.component_drain(id, self.component_types(id))
    }

    pub fn component_drain_insert<'a>(&self, source_id: EntityId, dest_id: EntityId, entity_store: &'a mut EntityStore) -> ComponentDrainInsert<'a> {
        entity_store.component_drain_insert(source_id, dest_id, self.component_types(source_id))
    }
}

pub struct RemoveEntityIter {
    id: EntityId,
    iter: ComponentTypeSetIter,
}

impl Iterator for RemoveEntityIter {
    type Item = EntityChange;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|t| EntityChange::Remove(self.id, t))
    }
}
