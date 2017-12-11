#![allow(dead_code)]

use enum_primitive::FromPrimitive;
use super::{constants, ComponentType};

const BITMAP_BITS: usize = 64;
const NUM_BITMAPS: usize = 1 + (constants::NUM_COMPONENT_TYPES - 1) / BITMAP_BITS;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ComponentTypeSet {
    bitmaps: [u64; NUM_BITMAPS],
}

impl ComponentTypeSet {
    pub fn new() -> Self {
        ComponentTypeSet {
            bitmaps: [0; NUM_BITMAPS],
        }
    }

    pub fn is_empty(&self) -> bool {
        for b in self.bitmaps.iter() {
            if *b != 0 {
                return false;
            }
        }
        true
    }

    pub fn insert(&mut self, component_type: ComponentType) {
        self.bitmaps[(component_type as usize) / BITMAP_BITS]
            |= 1 << ((component_type as usize) % BITMAP_BITS);
    }

    pub fn remove(&mut self, component_type: ComponentType) {
        self.bitmaps[(component_type as usize) / BITMAP_BITS]
            &= !(1 << ((component_type as usize) % BITMAP_BITS));
    }

    pub fn contains(&self, component_type: ComponentType) -> bool {
        self.bitmaps[(component_type as usize) / BITMAP_BITS] &
            (1 << ((component_type as usize % BITMAP_BITS))) != 0
    }

    pub fn iter(&self) -> ComponentTypeSetIter {
        ComponentTypeSetIter {
            bitmaps: self.bitmaps,
            index: 0,
        }
    }
}

pub struct ComponentTypeSetIter {
    bitmaps: [u64; NUM_BITMAPS],
    index: usize,
}

impl ComponentTypeSetIter {
    pub fn empty() -> Self {
        ComponentTypeSetIter {
            bitmaps: [0; NUM_BITMAPS],
            index: NUM_BITMAPS,
        }
    }
}

impl Iterator for ComponentTypeSetIter {
    type Item = ComponentType;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < NUM_BITMAPS &&
            self.bitmaps[self.index] == 0
        {
            self.index += 1;
        }
        if self.index == NUM_BITMAPS {
            return None;
        }

        let trailing = self.bitmaps[self.index].trailing_zeros();
        self.bitmaps[self.index] &= !(1 << trailing);
        let component_type_num = trailing + (self.index as u32) * BITMAP_BITS as u32;
        let component_type = ComponentType::from_u32(component_type_num)
            .expect("Failed to form ComponentType from ComponentTypeSetIter");

        Some(component_type)
    }
}
