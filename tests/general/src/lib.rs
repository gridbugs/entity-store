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
