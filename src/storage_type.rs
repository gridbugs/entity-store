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
}
