// imports

use std::sync::{RwLock, RwLockWriteGuard};

use lazy_static::lazy_static;

use crate::index::{Index, Item};
use crate::traits::Initialise;
use crate::types::ID;


pub fn handle (req: super::Req) -> super::Resp {
    use super::types::*;

    lazy_static! {
        static ref INDEX_MUTEX: RwLock<crate::index::Default> = RwLock::new(crate::index::default());
        static ref STORE_MUTEX: RwLock<crate::store::Store> = RwLock::new(crate::store::Store::new());
    }

    let mut index = INDEX_MUTEX.write().expect("failed to unlock index");
    let mut store = STORE_MUTEX.write().expect("failed to unlock store");

    match req {
        Req::Click(ClickReq { id, query }) => Resp::Click(handle_click(index, &id, &query)),
        Req::Search(SearchReq { query }) => Resp::Query(handle_search(index, store, &query)),
        Req::Write(WriteReq { id, item }) => Resp::Write(handle_write(index, store, &id, &item)),
    }
}


fn handle_click (index: RwLockWriteGuard<crate::index::Default>, id: &ID, query: &String) -> super::types::ClickResp {
    use super::types::*;

    // Write click to index
    let resp = match index.click(id, query) {
        Ok(()) => ClickResp::Success(()),
        Err(e) => ClickResp::Error(e.to_string())
    };

    resp
}

fn handle_search (
    index: RwLockWriteGuard<crate::index::Default>,
    store: RwLockWriteGuard<crate::store::Store>,
    query: &String,
) -> super::types::SearchResp {
    use super::types::*;

    // Search index for query
    let ids = match index.search(query) {
        Ok(ids) => ids,
        Err(e) => return SearchResp::Error(e.to_string()),
    };

    let items = store.list(&ids);

    SearchResp::Success(items)
}

fn handle_write (
    mut index: RwLockWriteGuard<crate::index::Default>,
    mut store: RwLockWriteGuard<crate::store::Store>,
    id: &Option<ID>,
    item: &Item,
) -> super::types::WriteResp {
    use super::types::*;

    // Persist the item in the store. We take a copy of the item
    // here, for the first and only time, as it must outlive the
    // request. This does not apply below, where only substrings
    // are copied as needed.
    let id = store.set(id.as_ref(), item.clone());

    // Index the item by its ID. This means that any parts which
    // are vital to serve a query are copied, but the store does
    // not have a full copy of the item. If the index is changed
    // so as to require previously-unindexed data, it can replay
    // the process by retrieving the item from the store.
    let resp = match index.write(&id, &item) {
        Ok(v) => WriteResp::Success(v),
        Err(e) => WriteResp::Error(e.to_string())
    };

    resp
}