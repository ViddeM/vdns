use std::fmt::Display;

use crate::{common::ip_address::IPV4Address, messages::parsing::Reader};

#[derive(Debug, Clone)]
pub struct A {
    address: IPV4Address,
}

impl A {
    pub fn parse(reader: &mut Reader) -> Option<Self> {
        Some(A {
            address: IPV4Address::parse(reader)?,
        })
    }
}

impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}
