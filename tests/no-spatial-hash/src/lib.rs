#[macro_use]
extern crate entity_store_helper;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod entity_store {
    include_entity_store!("entity_store.rs");
}
