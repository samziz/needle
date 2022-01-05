// imports

use serde::{Deserialize, Serialize};

use crate::index::Item;
use crate::types::ID;


#[derive(Deserialize)]
pub struct ClickReq {
    pub id: ID,
    pub query: String,
}

#[derive(Serialize)]
pub enum ClickResp {
    Error(String),
    Success(()),
}


#[derive(Deserialize)]
pub struct SearchReq {
    pub query: String,
}

#[derive(Serialize)]
pub enum SearchResp {
    Error(String),
    Success(Vec<Item>),
}


#[derive(Deserialize)]
pub struct WriteReq {
    pub id: Option<ID>,
    pub item: Item,
}

#[derive(Serialize)]
pub enum WriteResp {
    Error(String),
    Success(()),
}


#[derive(Deserialize)]
pub enum Req {
    Click(ClickReq),
    Search(SearchReq),
    Write(WriteReq),
}

#[derive(Serialize)]
pub enum Resp {
    Click(ClickResp),
    Query(SearchResp),
    Write(WriteResp),
}