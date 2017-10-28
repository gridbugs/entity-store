#[derive(Clone, Copy, Debug)]
pub enum StorageType {
    Vector,
    Hash,
    BTree,
}

use self::StorageType::*;

pub const ALL: &[StorageType] = &[
    Vector,
    Hash,
    BTree,
];

impl StorageType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "vector" => Some(Vector),
            "hash" => Some(Hash),
            "btree" => Some(BTree),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Vector => "vector",
            Hash => "hash",
            BTree => "btree",
        }
    }

    pub fn to_map_type(self) -> &'static str {
        match self {
            Vector => "EntityVecMap",
            Hash => "EntityHashMap",
            BTree => "EntityBTreeMap",
        }
    }

    pub fn to_set_type(self) -> &'static str {
        match self {
            Vector => "EntityVecSet",
            Hash => "EntityHashSet",
            BTree => "EntityBTreeSet",
        }
    }
}
