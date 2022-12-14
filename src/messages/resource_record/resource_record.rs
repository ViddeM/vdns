use std::fmt::Display;

use crate::{common::rr_type::RRType, messages::parsing::Reader};

pub struct ResourceRecord {
    name: String,
    record_type: RRType,
    class: u16,     // TODO:?
    ttl: u32,       // Time to live in seconds
    rd_length: u16, // Length of the rdata field
    rdata: Vec<u8>,
}

impl ResourceRecord {
    pub fn parse(reader: &mut Reader) -> Option<ResourceRecord> {
        println!(
            "Parsing Resource record from {:#?}",
            reader.peek_remaining_bytes()
        );

        None
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        todo!("Eeerh not done");
    }
}

impl Display for ResourceRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Display not implemented")
    }
}
