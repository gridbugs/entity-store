#![allow(dead_code)]

use std::collections::{hash_map, hash_set, btree_map, btree_set};
use super::flat_collections::{FlatMapIter, FlatMapIterMut, FlatSetIter, FlatMapKeys};
use super::vec_collections::{VecMapIter, VecMapIterMut, VecSetIter, VecMapKeys};
use super::id::EntityIdRaw;

pub type EntityHashMapIter<'a, T> = hash_map::Iter<'a, EntityIdRaw, T>;
pub type EntityHashMapIterMut<'a, T> = hash_map::IterMut<'a, EntityIdRaw, T>;
pub type EntityHashMapKeys<'a, T> = hash_map::Keys<'a, EntityIdRaw, T>;
pub type EntityHashSetIter<'a> = hash_set::Iter<'a, EntityIdRaw>;
pub type EntityBTreeMapIter<'a, T> = btree_map::Iter<'a, EntityIdRaw, T>;
pub type EntityBTreeMapIterMut<'a, T> = btree_map::IterMut<'a, EntityIdRaw, T>;
pub type EntityBTreeMapKeys<'a, T> = btree_map::Keys<'a, EntityIdRaw, T>;
pub type EntityBTreeSetIter<'a> = btree_set::Iter<'a, EntityIdRaw>;
pub type EntityFlatMapIter<'a, T> = FlatMapIter<'a, T>;
pub type EntityFlatMapIterMut<'a, T> = FlatMapIterMut<'a, T>;
pub type EntityFlatMapKeys<'a, T> = FlatMapKeys<'a, T>;
pub type EntityFlatSetIter<'a> = FlatSetIter<'a>;
pub type EntityVecMapIter<'a, T> = VecMapIter<'a, EntityIdRaw, T>;
pub type EntityVecMapIterMut<'a, T> = VecMapIterMut<'a, EntityIdRaw, T>;
pub type EntityVecMapKeys<'a, T> = VecMapKeys<'a, EntityIdRaw, T>;
pub type EntityVecSetIter<'a> = VecSetIter<'a, EntityIdRaw>;
