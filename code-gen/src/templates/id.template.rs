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

impl<'a> EntityId<'a> {
    pub fn to_free(self) -> EntityIdToFree {
        EntityIdToFree {
            raw: self.raw,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EntityIdToFree {
    pub(super) raw: EntityIdRaw,
}
