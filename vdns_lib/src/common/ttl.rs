use std::{fmt::Display, time::Duration};

use crate::messages::parsing::Reader;

#[derive(Debug, Clone)]
pub enum TTL {
    NoCache, // The request should not be cached
    Cache(Duration),
}

impl TTL {
    pub fn parse(reader: &mut Reader) -> Option<TTL> {
        let seconds = reader.read_u32()?;
        Some(match seconds {
            0 => TTL::NoCache,
            val => TTL::Cache(Duration::from_secs(val as u64)),
        })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        let val = match self {
            TTL::NoCache => 0,
            TTL::Cache(duration) => duration.as_secs() as u16,
        };
        let [b1, b2] = val.to_be_bytes();
        buf.push(b1);
        buf.push(b2);
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
