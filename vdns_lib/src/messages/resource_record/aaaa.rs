use std::{fmt::Display, net::Ipv6Addr};

use crate::messages::parsing::Reader;

#[derive(Debug, Clone)]
pub struct AAAA {
    address: Ipv6Addr,
}

impl AAAA {
    pub fn parse(reader: &mut Reader) -> Option<Self> {
        Some(AAAA {
            address: Ipv6Addr::from(reader.read_u128()?),
        })
    }
}

impl Display for AAAA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}
