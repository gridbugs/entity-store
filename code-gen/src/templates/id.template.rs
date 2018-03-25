use std::marker::PhantomData;

pub type EntityIdRaw = {{ id_type }};

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

impl<'a> EntityId<'a> {
    pub fn raw(self) -> EntityIdRaw {
        self.raw
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct EntityIdRuntimeChecked {
    pub(super) raw: EntityIdRaw,
    pub(super) free_count: u64,
}
