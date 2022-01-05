// imports

use std::collections::HashMap;

use crate::traits::Initialise;
use crate::types::ID;


pub struct ClickWrapper<I>
    where I: crate::index::Index {
    data: HashMap<String, ID>,
    inner: I,
}

impl<I> Initialise for ClickWrapper<I> where I: crate::index::Index {
    fn new () -> Self {
        Self {
            data: HashMap::new(),
            inner: I::new(),
        }
    }
}

impl<I> ClickWrapper<I> where I: crate::index::Index {
    pub fn click (&self, id: &crate::types::ID, query: &String) -> Result<(), std::io::Error> {
        self.data.insert(query.clone(), id.clone());
        Ok(())
    }
}

impl<I> crate::index::Index for ClickWrapper<I> where I: crate::index::Index {
    fn search (&self, query: &String) -> Result<Vec<ID>, std::io::Error> {
        self.inner.search(query)
    }

    fn write (&mut self, id: &crate::types::ID, item: &crate::index::Item) -> Result<(), std::io::Error> {
        self.inner.write(id, item)
    }
}

impl<I> super::Wrapper for ClickWrapper<I> where I: crate::index::Index {}