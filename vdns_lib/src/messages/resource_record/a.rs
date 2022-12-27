use std::{fmt::Display, net::Ipv4Addr};

use crate::{
    common::parse_error::ParseResult,
    messages::{parsing::Reader, serializing::Writer},
};

#[derive(Debug, Clone)]
pub struct A {
    address: Ipv4Addr,
}

impl A {
    pub fn parse(reader: &mut Reader) -> ParseResult<Self> {
        Ok(A {
            address: Ipv4Addr::from(reader.read_u32()?),
        })
    }

    pub fn serialize(&self, writer: &mut Writer) {
        for b in self.address.octets().iter() {
            writer.write_u8(*b);
        }
    }
}

impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}
