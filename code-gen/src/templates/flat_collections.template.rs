#![allow(dead_code)]

use std::mem;
use std::slice;
use std::iter;
use super::id::EntityIdRaw;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatMap<T> {
    elements: Vec<Option<T>>,
}

impl<T> FlatMap<T> {
    pub fn new() -> Self {
        FlatMap {
            elements: Vec::new(),
        }
    }

    pub fn remove(&mut self, index: &EntityIdRaw) -> Option<T> {
        if (*index as usize) >= self.elements.len() {
            return None;
        }

        mem::replace(&mut self.elements[*index as usize], None)
    }

    pub fn get(&self, index: &EntityIdRaw) -> Option<&T> {
        self.elements.get(*index as usize).and_then(Option::as_ref)
    }

    pub fn get_mut(&mut self, index: &EntityIdRaw) -> Option<&mut T> {
        self.elements.get_mut(*index as usize).and_then(Option::as_mut)
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn contains_key(&self, index: &EntityIdRaw) -> bool {
        self.get(index).is_some()
    }

    pub fn iter(&self) -> FlatMapIter<T> {
        FlatMapIter {
            iter: self.elements.iter().enumerate(),
        }
    }

    pub fn keys(&self) -> FlatMapKeys<T> {
        FlatMapKeys(self.iter())
    }

    pub fn entry(&mut self, index: &EntityIdRaw) -> FlatMapEntry<T> {
        if self.contains_key(index) {
            let value = self.get_mut(index).unwrap();
            FlatMapEntry::Occupied(value)
        } else {
            FlatMapEntry::Vacant {
                map: self,
                index: *index as usize,
            }
        }
    }

    fn insert_raw(&mut self, index: usize, component: T) -> Option<T> {
        if let Some(value) = self.elements.get_mut(index) {
            return mem::replace(value, Some(component));
        }

        while !self.elements.len() < index {
            self.elements.push(None);
        }
        self.elements.push(Some(component));

        None
    }

    pub fn insert(&mut self, index: EntityIdRaw, component: T) -> Option<T> {
        self.insert_raw(index as usize, component)
    }
}

impl<T> Default for FlatMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FlatMapIter<'a, T: 'a> {
    iter: iter::Enumerate<slice::Iter<'a, Option<T>>>,
}

impl<'a, T: 'a> Iterator for FlatMapIter<'a, T> {
    type Item = (EntityIdRaw, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, maybe_value)) = self.iter.next() {
            if let Some(value) = maybe_value.as_ref() {
                return Some((index as EntityIdRaw, value));
            }
        }

        None
    }
}

pub struct FlatMapKeys<'a, T: 'a>(FlatMapIter<'a, T>);
impl<'a, T: 'a> Iterator for FlatMapKeys<'a, T> {
    type Item = EntityIdRaw;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(raw, _)| raw)
    }
}

pub enum FlatMapEntry<'a, T: 'a> {
    Occupied(&'a mut T),
    Vacant {
        map: &'a mut FlatMap<T>,
        index: usize,
    },
}

impl<'a, T> FlatMapEntry<'a, T> {
    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            FlatMapEntry::Occupied(v) => v,
            FlatMapEntry::Vacant { map, index } => {
                map.insert_raw(index, default);
                map.elements[index].as_mut().unwrap()
            }
        }
    }
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        match self {
            FlatMapEntry::Occupied(v) => v,
            FlatMapEntry::Vacant { map, index } => {
                map.insert_raw(index, default());
                map.elements[index].as_mut().unwrap()
            }
        }

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatSet {
    elements: Vec<u64>,
}

impl FlatSet {
    pub fn new() -> Self {
        FlatSet {
            elements: Vec::new(),
        }
    }

    fn index_mask(id: usize) -> (usize, u64) {
        let index = id / 64;
        let offset = (id % 64) as u32;
        let mask = (1 as u64) << offset;

        (index, mask)
    }

    pub fn insert(&mut self, index: EntityIdRaw) -> bool {
        let (index, mask) = Self::index_mask(index as usize);

        if let Some(bits) = self.elements.get_mut(index) {
            let current = *bits & mask != 0;
            *bits |= mask;
            return current;
        }

        self.elements.resize(index, 0);
        self.elements.push(mask);

        false
    }

    pub fn remove(&mut self, index: &EntityIdRaw) -> bool {
        let (index, mask) = Self::index_mask(*index as usize);

        if let Some(bits) = self.elements.get_mut(index) {
            let current = *bits & mask != 0;
            *bits &= !mask;
            current
        } else {
            false
        }
    }

    pub fn contains(&self, index: &EntityIdRaw) -> bool {
        let (index, mask) = Self::index_mask(*index as usize);

        if let Some(bits) = self.elements.get(index) {
            bits & mask != 0
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn is_empty(&self) -> bool {
        for bits in self.elements.iter() {
            if *bits != 0 {
                return false;
            }
        }
        true
    }

    pub fn iter(&self) -> FlatSetIter {
        let mut iter = self.elements.iter();
        FlatSetIter {
            current: iter.next().map(Clone::clone).unwrap_or(0),
            iter,
            base: 0,
        }
    }
}

impl Default for FlatSet {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FlatSetIter<'a> {
    iter: slice::Iter<'a, u64>,
    current: u64,
    base: usize,
}

impl<'a> Iterator for FlatSetIter<'a> {
    type Item = EntityIdRaw;
    fn next(&mut self) -> Option<Self::Item> {
        while self.current == 0 {
            if let Some(current) = self.iter.next() {
                self.current = *current;
                self.base += 64;
            } else {
                return None;
            }
        }

        let trailing = self.current.trailing_zeros() as usize;
        self.current &= !(1 << trailing);

        Some((self.base + trailing) as EntityIdRaw)
    }
}
