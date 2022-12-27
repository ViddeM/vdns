use std::{fmt::Display, time::Duration};

use crate::messages::{parsing::Reader, serializing::Writer};

use super::parse_error::ParseResult;

#[derive(Debug, Clone)]
pub enum TTL {
    NoCache, // The request should not be cached
    Cache(Duration),
}

impl TTL {
    pub fn parse(reader: &mut Reader) -> ParseResult<TTL> {
        let seconds = reader.read_u32()?;
        Ok(match seconds {
            0 => TTL::NoCache,
            val => TTL::Cache(Duration::from_secs(val as u64)),
        })
    }

    pub fn serialize(&self, writer: &mut Writer) {
        let val = match self {
            TTL::NoCache => 0,
            TTL::Cache(duration) => duration.as_secs() as u32,
        };
        let [b1, b2, b3, b4] = val.to_be_bytes();
        writer.write_u8(b1);
        writer.write_u8(b2);
        writer.write_u8(b3);
        writer.write_u8(b4);
    }
}

impl Display for TTL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TTL::NoCache => "No Cache (0)".to_string(),
                TTL::Cache(duration) => format!("{} seconds", duration.as_secs()),
            }
        )
    }
}
