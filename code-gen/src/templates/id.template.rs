use std::marker::PhantomData;

pub(super) type EntityIdRaw = {{ id_type }};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EntityWit<'a>(PhantomData<&'a ()>);

impl<'w> EntityWit<'w> {
    pub(super) fn new() -> Self {
        EntityWit(PhantomData)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EntityId<'a> {
    pub(super) raw: EntityIdRaw,
    pub(super) wit: EntityWit<'a>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct EntityIdToFree {
    pub(super) raw: EntityIdRaw,
    pub(super) free_count: u64,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct EntityIdToStore {
    pub(super) raw: EntityIdRaw,
    pub(super) free_count: u64,
}
