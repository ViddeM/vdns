use std::{fmt::Display, net::Ipv6Addr};

use crate::{
    common::parse_error::ParseResult,
    messages::{parsing::Reader, serializing::write_u8},
};

#[derive(Debug, Clone)]
pub struct AAAA {
    address: Ipv6Addr,
}

impl AAAA {
    pub fn parse(reader: &mut Reader) -> ParseResult<Self> {
        Ok(AAAA {
            address: Ipv6Addr::from(reader.read_u128()?),
        })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        for b in self.address.octets().iter() {
            write_u8(buf, *b);
        }
    }
}

impl Display for AAAA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}
