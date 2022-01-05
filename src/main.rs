#![allow(incomplete_features)]
#![feature(asm)]
#![feature(assert_matches)]
#![feature(associated_type_defaults)]
#![feature(const_generics_defaults)]
#![feature(const_ip)]
#![feature(const_ip_v4)]
#![feature(generic_associated_types)]
#![feature(generic_const_exprs)]
#![feature(in_band_lifetimes)]
#![feature(inherent_associated_types)]
#![feature(int_abs_diff)]
#![feature(more_qualified_paths)]
#![feature(never_type)]
#![feature(portable_simd)]
#![feature(test)] 
#![feature(type_alias_impl_trait)] 

// imports

extern crate test;

mod consts;
mod core;
mod index;
mod server;
mod storage;
mod store;
mod traits;
mod types;

use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use server::{Server, handlers};


#[tokio::main]
async fn main () {
    serve().await;
}

async fn serve () {
    let IPV6_ADDR: Ipv6Addr = Ipv6Addr::from(consts::SERVER_IPV6_ADDR);
    let SOCK_ADDR: SocketAddr = SocketAddr::new(IpAddr::V6(IPV6_ADDR), consts::SERVER_PORT);

    let server = server::default();
    
    server.listen(SOCK_ADDR, handlers::handle).await;
}

#[cfg(test)]
mod integration_tests {
    use std::thread;

    #[test]
    fn test_basic () {
        // setup
        setup();

        todo!();
    }

    fn setup () {
        use crate::main;
        thread::spawn(move || main());
    }
}