use std::{fmt::Display, net::Ipv4Addr};

use crate::messages::parsing::Reader;

#[derive(Debug, Clone)]
pub struct A {
    address: Ipv4Addr,
}

impl A {
    pub fn parse(reader: &mut Reader) -> Option<Self> {
        Some(A {
            address: Ipv4Addr::from(reader.read_u32()?),
        })
    }
}

impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}
