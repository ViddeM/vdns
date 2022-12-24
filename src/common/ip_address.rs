use std::fmt::Display;

use crate::messages::parsing::Reader;

#[derive(Debug, Clone)]
pub struct IPV4Address {
    address: u32,
}

impl IPV4Address {
    pub fn parse(reader: &mut Reader) -> Option<Self> {
        Some(IPV4Address {
            address: reader.read_u32()?,
        })
    }
}

impl Display for IPV4Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            (self.address >> 24) as u8,
            (self.address >> 16) as u8,
            (self.address >> 8) as u8,
            (self.address) as u8,
        )
    }
}
