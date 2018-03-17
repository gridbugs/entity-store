#[macro_use]
extern crate entity_store_helper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate grid_2d;

pub mod entity_store {
    include_entity_store!("entity_store.rs");
}

use grid_2d::{Size, Coord};
use entity_store::*;

fn main() {
    let (mut store, mut wit) = EntityStore::new(Size::new(10, 10));

    let to_free = {
        let mut some_id = None;
        for (id, coord) in store.iter_coord(&wit) {
            let c = store.get_coord(id).unwrap();

            some_id = Some(id);
        }

        let some_id = some_id.unwrap();

        store.insert_coord(some_id, Coord::new(0, 0));

        some_id.to_free()
    };

    store.remove_entity(&mut wit, to_free);
}
