use std::net::IpAddr;

pub mod v4 {
    use std::net::Ipv4Addr;

    use super::*;

    pub fn from<F>(v: F) -> IpAddr
        where F: Into<Ipv4Addr> {
        IpAddr::V4(v.into())
    }
}