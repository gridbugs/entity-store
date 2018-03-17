use super::id::EntityIdRaw;
use super::entity_store_raw::EntityFlatMap;
use super::component_type_set::ComponentTypeSet;
use super::component::ComponentType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentTable {
    table: EntityFlatMap<ComponentTypeSet>,
}

impl EntityComponentTable {
    pub fn new() -> Self {
        Self {
            table: EntityFlatMap::new(),
        }
    }
    {% for key, component in components %}
        pub fn insert_{{ key }}(&mut self, id: EntityIdRaw) {
            self.table.entry(&id).or_insert_with(ComponentTypeSet::new)
                .insert(ComponentType::{{ component.name }});
        }
        pub fn remove_{{ key }}(&mut self, id: EntityIdRaw) {
            self.table.entry(&id).or_insert_with(ComponentTypeSet::new)
                .remove(ComponentType::{{ component.name }});
        }
    {% endfor %}
    pub fn remove(&mut self, id: EntityIdRaw) -> Option<ComponentTypeSet> {
        self.table.remove(&id)
    }
}
