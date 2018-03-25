#![allow(dead_code)]

use std::mem;
use std::slice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecMap<K, V> {
    elements: Vec<(K, V)>,
}

pub struct VecMapIter<'a, K: 'a, V: 'a> {
    iter: slice::Iter<'a, (K, V)>,
}

pub struct VecMapIterMut<'a, K: 'a, V: 'a> {
    iter: slice::IterMut<'a, (K, V)>,
}

impl<'a, K, V> Iterator for VecMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&(ref k, ref v)| (k, v))
    }
}

impl<'a, K, V> Iterator for VecMapIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&mut (ref k, ref mut v)| (k, v))
    }
}

pub struct VecMapKeys<'a, K: 'a, V: 'a>(VecMapIter<'a, K, V>);

impl<'a, K, V> Iterator for VecMapKeys<'a, K, V> {
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, _)| k)
    }
}

impl<K: Eq, V> Default for VecMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq, V> VecMap<K, V> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.elements.iter().find(|&&(ref k, _)| k.eq(key)).map(|&(_, ref v)| v)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.elements.iter().any(|&(ref k, _)| k.eq(key))
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(index) = self.elements.iter().position(|&(ref k, _)| k.eq(key)) {
            Some(self.elements.swap_remove(index).1)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(index) = self.elements.iter().position(|&(ref k, _)| k.eq(&key)) {
            Some(mem::replace(&mut self.elements[index].1, value))
        } else {
            self.elements.push((key, value));
            None
        }
    }

    pub fn iter(&self) -> VecMapIter<K, V> {
        VecMapIter {
            iter: self.elements.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> VecMapIterMut<K, V> {
        VecMapIterMut {
            iter: self.elements.iter_mut(),
        }
    }

    pub fn keys(&self) -> VecMapKeys<K, V> {
        VecMapKeys(self.iter())
    }

    pub fn first(&self) -> Option<(&K, &V)> {
        self.elements.first().map(|&(ref k, ref v)| (k, v))
    }

    pub fn first_key(&self) -> Option<&K> {
        self.elements.first().map(|&(ref k, _)| k)
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecSet<T> {
    elements: Vec<T>,
}

pub type VecSetIter<'a, T> = slice::Iter<'a, T>;

impl<T: Eq> Default for VecSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq> VecSet<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn contains(&self, t: &T) -> bool {
        self.elements.contains(t)
    }

    pub fn insert(&mut self, t: T) -> bool {
        if self.elements.contains(&t) {
            false
        } else {
            self.elements.push(t);
            true
        }
    }

    pub fn remove(&mut self, t: &T) -> bool {
        if let Some(index) = self.elements.iter().position(|other| t.eq(other)) {
            self.elements.swap_remove(index);
            true
        } else {
            false
        }
    }

    pub fn first(&self) -> Option<&T> {
        self.elements.first()
    }

    pub fn iter(&self) -> VecSetIter<T> {
        self.elements.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}
