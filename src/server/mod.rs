// exports

pub mod handlers;
pub mod types; 

pub use types::{Req, Resp};

// imports

use std::net::SocketAddr;

use async_trait::async_trait;


#[async_trait]
pub trait Server {
    type Req = types::Req;
    type Resp = types::Resp;
    type Handler = fn(Self::Req) -> Self::Resp;

    async fn listen (self, addr: SocketAddr, handler: Self::Handler);
}


pub type Default = warp::Server;

pub fn default () -> Default {
    Default {}
}


mod warp {
    use std::net::SocketAddr;

    use async_trait::async_trait;
    use warp::Filter;

    use crate::types::ID;
    use super::Server as ServerTrait;
    use super::types;

    
    pub struct Server;

    #[async_trait]
    impl ServerTrait for Server {
        async fn listen (self, addr: SocketAddr, handler: Self::Handler) {
            use types::Req::*;
            use warp::reply::json;

            let get_query = warp::get()
                .and(warp::path!("query" / String))
                .and(warp::path::end())
                .map(move |query| json(&handler(Search(types::SearchReq { query }))));

            let post_click = warp::post()
                .and(warp::path!("click" / ID / String))
                .and(warp::path::end())
                .map(move |id, query| json(&handler(Click(types::ClickReq { id, query }))));
            
            let post_write = warp::post()
                .and(warp::path!("write"))
                .and(warp::body::json())
                .and(warp::path::end())
                .map(move |msg: types::WriteReq| json(&handler(Write(msg))));

            let endpoints = get_query
                .or(post_click)
                .or(post_write);

            warp::serve(endpoints)
                .run(addr)
                .await
        }
    }
}
