use super::{EntityVecMap, ComponentTypeSet, ComponentTypeSetIter, EntityChange, EntityId};

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

    pub fn components(&self, id: EntityId) -> ComponentTypeSetIter {
        self.0.get(&id)
            .map(|s| s.iter())
            .unwrap_or_else(|| ComponentTypeSetIter::empty())
    }

    pub fn remove_entity(&self, id: EntityId) -> RemoveEntityIter {
        RemoveEntityIter {
            id,
            iter: self.components(id),
        }
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
