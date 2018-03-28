#![allow(dead_code)]

use std::marker::PhantomData;

pub type EntityIdRaw = {{ id_type }};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct EntityWit(());

impl EntityWit {
    pub(super) fn new() -> Self {
        EntityWit(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EntityId<'w> {
    pub(super) raw: EntityIdRaw,
    pub(super) phantom: PhantomData<&'w ()>,
}

impl<'w> EntityId<'w> {
    pub fn raw(self) -> EntityIdRaw {
        self.raw
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct EntityIdRuntimeChecked {
    pub(super) raw: EntityIdRaw,
    pub(super) free_count: u64,
}
