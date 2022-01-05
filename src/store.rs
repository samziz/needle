// imports

use std::collections::HashMap;

use crate::index::Item;
use crate::traits::Initialise;
use crate::types::ID;


pub struct Store <
    Hasher = t1ha::T1haBuildHasher,
> {
    /// `entries` is an associative array, mapping an item's ID
    /// to its corresponding data. Remember that each item is a
    /// tuple containing the external ID (which are different
    /// from our internal, hash-derived IDs) and the item.
    entries: HashMap<ID, Item, Hasher>,
}

impl Initialise for Store {
    fn new () -> Self {
        Self {
            entries: t1ha::T1haHashMap::default(),
        }
    }
}

impl Store {
    pub fn get (&self, id: &ID) -> Option<&Item> {
        self.entries.get(id)
    }

    pub fn list (&self, ids: &[ID]) -> Vec<Item> {
        self.entries
            .iter()
            .filter(|(id,..)| ids.contains(id))
            .map(|(..,item)| item)
            .cloned()
            .collect()
    }

    pub fn set (&mut self, id: Option<&ID>, item: Item) -> ID {
        let id = match id {
            Some(&v) => v,
            None => uuid::Uuid::new_v4().to_u128_le(),
        };

        self.entries.insert(id, item);

        id
    }
}