#[macro_use]
extern crate entity_store_helper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate maplit;

pub mod entity_store {
    include_entity_store!("entity_store.rs");
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;
    use entity_store::*;

    #[test]
    fn remove_entity() {

        let (mut store, mut wit) = EntityStore::new(Size::new(10, 10));

        let id_a = {
            let id_a = store.allocate_entity_id(&wit);

            store.insert_coord(id_a, Coord::new(1, 1));
            store.insert_opacity(id_a, 3);

            store.create_runtime_checked_entity_id(id_a)
        };

        {
            let id_a = store.check_entity_id(&wit, id_a).unwrap();
            assert_eq!(store.get_coord(id_a), Some(&Coord::new(1, 1)));
        }

        store.remove_entity(&mut wit, id_a);

        {
            assert!(store.check_entity_id(&wit, id_a).is_none());
        }
    }

    #[test]
    fn transfer_entity() {
        let (mut store_a, wit_a) = EntityStore::new(Size::new(10, 10));
        let (mut store_b, wit_b) = EntityStore::new(Size::new(10, 10));

        let id_a = store_a.allocate_entity_id(&wit_a);

        store_a.insert_coord(id_a, Coord::new(1, 1));
        store_a.insert_opacity(id_a, 3);
        store_a.insert_player(id_a);
        store_a.insert_solid(id_a);

        let id_b = store_b.allocate_entity_id(&wit_b);

        assert_eq!(store_a.get_coord(id_a), Some(&Coord::new(1, 1)));
        assert_eq!(store_a.get_opacity(id_a), Some(&3));
        assert!(store_a.contains_player(id_a));
        assert!(store_a.contains_solid(id_a));

        assert!(store_b.get_coord(id_b).is_none());
        assert!(store_b.get_opacity(id_b).is_none());
        assert!(!store_b.contains_player(id_b));
        assert!(!store_b.contains_solid(id_b));

        for value in store_a.drain_entity_components(id_a) {
            store_b.insert(id_b, value);
        }

        assert!(store_a.get_coord(id_a).is_none());
        assert!(store_a.get_opacity(id_a).is_none());
        assert!(!store_a.contains_player(id_a));
        assert!(!store_a.contains_solid(id_a));

        assert_eq!(store_b.get_coord(id_b), Some(&Coord::new(1, 1)));
        assert_eq!(store_b.get_opacity(id_b), Some(&3));
        assert!(store_b.contains_player(id_b));
        assert!(store_b.contains_solid(id_b));
    }

    #[test]
    fn spatial_hash_maintains_aggregates() {

        let (mut store, wit) = EntityStore::new(Size::new(10, 10));

        let id_a = store.allocate_entity_id(&wit);

        store.insert_coord(id_a, Coord::new(1, 1));
        store.insert_solid(id_a);
        store.insert_opacity(id_a, 3);
        store.insert_player(id_a);

        {
            let cell = store.spatial_hash_get(Coord::new(1, 1)).unwrap();
            assert_eq!(cell.opacity_total, 3);
            assert_eq!(cell.solid_count, 1);
            assert_eq! {
                cell.player_set.iter(&wit).map(EntityId::raw).collect::<HashSet<_>>(),
                hashset![id_a.raw()],
            };
        }

        store.insert_coord(id_a, Coord::new(2, 2));

        {
            let cell = store.spatial_hash_get(Coord::new(1, 1)).unwrap();
            assert_eq!(cell.opacity_total, 0);
            assert_eq!(cell.solid_count, 0);
            assert_eq! {
                cell.player_set.iter(&wit).map(EntityId::raw).collect::<HashSet<_>>(),
                hashset![],
            };
        }

        {
            let cell = store.spatial_hash_get(Coord::new(2, 2)).unwrap();
            assert_eq!(cell.opacity_total, 3);
            assert_eq!(cell.solid_count, 1);
            assert_eq! {
                cell.player_set.iter(&wit).map(EntityId::raw).collect::<HashSet<_>>(),
                hashset![id_a.raw()],
            };
        }

        let id_b = store.allocate_entity_id(&wit);

        store.insert_coord(id_b, Coord::new(2, 2));
        store.insert_solid(id_b);
        store.insert_opacity(id_b, 2);
        store.insert_player(id_b);

        {
            let cell = store.spatial_hash_get(Coord::new(2, 2)).unwrap();
            assert_eq!(cell.opacity_total, 5);
            assert_eq!(cell.solid_count, 2);
            assert_eq! {
                cell.player_set.iter(&wit).map(EntityId::raw).collect::<HashSet<_>>(),
                hashset![id_a.raw(), id_b.raw()],
            };
        }

        store.insert_coord(id_b, Coord::new(1, 1));

        {
            let cell = store.spatial_hash_get(Coord::new(2, 2)).unwrap();
            assert_eq!(cell.opacity_total, 3);
            assert_eq!(cell.solid_count, 1);
            assert_eq! {
                cell.player_set.iter(&wit).map(EntityId::raw).collect::<HashSet<_>>(),
                hashset![id_a.raw()],
            };
        }
        {
            let cell = store.spatial_hash_get(Coord::new(1, 1)).unwrap();
            assert_eq!(cell.opacity_total, 2);
            assert_eq!(cell.solid_count, 1);
            assert_eq! {
                cell.player_set.iter(&wit).map(EntityId::raw).collect::<HashSet<_>>(),
                hashset![id_b.raw()],
            };
        }

        store.insert_coord(id_a, Coord::new(1, 1));

        {
            let cell = store.spatial_hash_get(Coord::new(2, 2)).unwrap();
            assert_eq!(cell.opacity_total, 0);
            assert_eq!(cell.solid_count, 0);
            assert_eq! {
                cell.player_set.iter(&wit).map(EntityId::raw).collect::<HashSet<_>>(),
                hashset![],
            };
        }
        {
            let cell = store.spatial_hash_get(Coord::new(1, 1)).unwrap();
            assert_eq!(cell.opacity_total, 5);
            assert_eq!(cell.solid_count, 2);
            assert_eq! {
                cell.player_set.iter(&wit).map(EntityId::raw).collect::<HashSet<_>>(),
                hashset![id_a.raw(), id_b.raw()],
            };
        }

        store.remove_solid(id_a);
        store.insert_opacity(id_a, 1);
        store.remove_opacity(id_b);
        store.remove_player(id_a);

        {
            let cell = store.spatial_hash_get(Coord::new(1, 1)).unwrap();
            assert_eq!(cell.opacity_total, 1);
            assert_eq!(cell.solid_count, 1);
            assert_eq! {
                cell.player_set.iter(&wit).map(EntityId::raw).collect::<HashSet<_>>(),
                hashset![id_b.raw()],
            };
        }
    }
}
