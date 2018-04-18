#[macro_use]
extern crate entity_store_helper;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod entity_store {
    include_entity_store!("entity_store.rs");
}

#[cfg(test)]
mod tests {

    use entity_store::*;

    #[test]
    fn basic() {
        let (mut store, wit) = EntityStore::new();
        let id_a = store.allocate_entity_id(&wit);
        store.insert_foo(id_a, 42);

        assert_eq!(store.get_foo(id_a), Some(&42));
    }
}
