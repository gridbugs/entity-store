use super::{ComponentValue, ComponentType, EntityId};

#[derive(Debug, Clone)]
pub enum EntityChange {
    Insert(EntityId, ComponentValue),
    Remove(EntityId, ComponentType),
}

pub mod insert {
    use super::{ComponentValue, EntityId, EntityChange};
    {% for key, component in components %}
        {% if component.type %}
            pub fn {{ key }}(id: EntityId, value: {{ component.type }}) -> EntityChange {
                EntityChange::Insert(id, ComponentValue::{{ component.name }}(value))
            }
        {% else %}
            pub fn {{ key }}(id: EntityId) -> EntityChange {
                EntityChange::Insert(id, ComponentValue::{{ component.name }})
            }
        {% endif %}
    {% endfor %}
}

pub mod remove {
    use super::{ComponentType, EntityId, EntityChange};
    {% for key, component in components %}
        pub fn {{ key }}(id: EntityId) -> EntityChange {
            EntityChange::Remove(id, ComponentType::{{ component.name }})
        }
    {% endfor %}
}
