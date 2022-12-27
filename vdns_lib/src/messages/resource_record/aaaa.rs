use std::{fmt::Display, net::Ipv6Addr};

use crate::{
    common::parse_error::ParseResult,
    messages::{parsing::Reader, serializing::Writer},
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

    pub fn serialize(&self, writer: &mut Writer) {
        for b in self.address.octets().iter() {
            writer.write_u8(*b);
        }
    }
}

impl Display for AAAA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}
