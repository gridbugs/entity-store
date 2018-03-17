#[derive(Clone, Copy, Debug)]
pub enum StorageType {
    Vector,
    Flat,
    Hash,
    BTree,
}

use self::StorageType::*;

pub const ALL: &[StorageType] = &[
    Vector,
    Flat,
    Hash,
    BTree,
];

impl StorageType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "vector" => Some(Vector),
            "flat" => Some(Flat),
            "hash" => Some(Hash),
            "btree" => Some(BTree),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Vector => "vector",
            Flat => "flat",
            Hash => "hash",
            BTree => "btree",
        }
    }

    pub fn to_map_type(self) -> &'static str {
        match self {
            Vector => "EntityVecMap",
            Flat => "EntityFlatMap",
            Hash => "EntityHashMap",
            BTree => "EntityBTreeMap",
        }
    }

    pub fn to_set_type(self) -> &'static str {
        match self {
            Vector => "EntityVecSet",
            Flat => "EntityFlatSet",
            Hash => "EntityHashSet",
            BTree => "EntityBTreeSet",
        }
    }

    pub fn to_map_iter_wrapper(self) -> &'static str {
        match self {
            Vector => "EntityIdAndValIterOfRef",
            Flat => "EntityIdAndValIterOfVal",
            Hash => "EntityIdAndValIterOfRef",
            BTree => "EntityIdAndValIterOfRef",
        }
    }

    pub fn to_set_iter_wrapper(self) -> &'static str {
        match self {
            Vector => "EntityIdIterOfRef",
            Flat => "EntityIdIterOfVal",
            Hash => "EntityIdIterOfRef",
            BTree => "EntityIdIterOfRef",
        }
    }

    pub fn to_map_iter(self) -> &'static str {
        match self {
            Vector => "EntityVecMapIter",
            Flat => "EntityFlatMapIter",
            Hash => "EntityHashMapIter",
            BTree => "EntityBTreeMapIter",
        }
    }
    pub fn to_map_keys(self) -> &'static str {
        match self {
            Vector => "EntityVecMapKeys",
            Flat => "EntityFlatMapKeys",
            Hash => "EntityHashMapKeys",
            BTree => "EntityBTreeMapKeys",
        }
    }
    pub fn to_set_iter(self) -> &'static str {
        match self {
            Vector => "EntityVecSetIter",
            Flat => "EntityFlatSetIter",
            Hash => "EntityHashSetIter",
            BTree => "EntityBTreeSetIter",
        }
    }
}
