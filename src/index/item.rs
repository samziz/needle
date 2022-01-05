// imports

use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
#[repr(transparent)]
pub struct Item (ItemInner);
pub type ItemInner = HashMap<String, KeyValue>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum KeyValue {
    Array(Vec<KeyValue>),
    Map(Item),
    Value(String),
}

impl Item {
    pub fn iter (&self) -> ItemIterator {
        ItemIterator {
            cur: KeyValue::Map(self.clone()),
            path: Vec::new(),
            offset: 0,
        }
    }

    #[cfg(test)]
    pub fn mock () -> Item {
        type Map = HashMap::<String, KeyValue>;
        Self (
            Map::from_iter([
                ("foo".to_owned(), KeyValue::Map(Self(Map::from_iter([
                    ("bar".to_owned(), KeyValue::Value("hello".to_owned())),
                    ("baz".to_owned(), KeyValue::Value("world".to_owned())),
                ].into_iter()))))
            ].into_iter()),
        )
    }
}

impl IntoIterator for Item {
    type IntoIter = ItemIterator;
    type Item = <ItemIterator as Iterator>::Item;


    fn into_iter (self) -> Self::IntoIter {
        Self::IntoIter {
            cur: KeyValue::Map(self.clone()),
            offset: 0,
            path: Vec::from([
                self
                    .0 
                    .keys().sorted()
                    .nth(0).unwrap()
                    .clone(),
            ]),
        }
    }
}

pub struct ItemIterator {
    cur: KeyValue,
    path: Vec<String>,
    offset: usize,
}

impl Iterator for ItemIterator {
    type Item = (Vec<String>, String);

    fn next (&mut self) -> Option<Self::Item> {
        self.get()
    }
}

impl ItemIterator {
    /// `get` is the primary method used for building an Item iterator.
    /// 
    /// ## Implementation
    /// 
    /// It maintains a field `cur`, which stores the current KeyValue being
    /// iterated over (it's always a KeyValue). At each iteration, it:
    /// 
    /// - Accesses the current KeyValue `cur`.
    /// - Checks the final element of `path` to determine which field
    ///   it is to access.
    /// - Looks up the field by name (if map) or number (if array).
    /// - Pattern matches to determine the type of the field.
    /// - If the field is a structure, it appends the name of the structure
    ///   to `path` and recurses.
    /// - If the field is a value, it returns that value.
    fn get (&mut self) -> Option<<Self as Iterator>::Item> {
        use self::KeyValue::*;

        let (cur, offset, path) =
            (self.cur.clone(), self.offset, self.path.clone());

        // Look up the field by name. At the first iteration, `cur` will be
        // the parent node, and `path` will be the field to access. This
        // pattern persists: `path` is one step ahead of `cur`.
        let _field = path
            .last()
            .expect("no field found; this indicates a code error");

        match cur {
            Array(a) =>
                a
                    .iter()
                    .nth(offset)
                    .and_then(|kv| {
                        self.cur = kv.clone();
                        self.get()
                    }),
            Map(Item(map)) =>
                map
                    .into_iter()
                    .sorted_by(|(a, _), (b, _)| a.cmp(b))
                    .nth(self.offset)
                    .and_then(|(_, value)| match value {
                        kv @ (Array(_) | Map(_)) => {
                            self.cur = kv;
                            self.get()
                        },
                        Value(v) => {
                            self.offset += 1;
                            Some((path, v))
                        },
                    }),
            Value(v) => Some((path, v)),
        }
    }
}

#[cfg(test)]
mod iter_tests {
    use std::assert;

    use super::*;

    #[test]
    fn iterates_without_error () {
        let item = Item::mock();
        let iter = item.into_iter();
        
        let results = iter.collect::<Vec<(Vec<String>, String)>>();
        assert!(results.len() == 2);
    }
}