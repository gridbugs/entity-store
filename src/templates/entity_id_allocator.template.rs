#![allow(dead_code)]
use super::EntityId;
use entity_store_helper::IdAllocator;

pub type EntityIdAllocator = IdAllocator<EntityId>;
