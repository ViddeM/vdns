use crate::common::formatting::indent_string;
use crate::messages::header::flags::Flags;
use crate::messages::parsing::read_u16;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct MessageHeader {
    id: u16,
    flags: Flags,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16,
}

impl MessageHeader {
    pub fn parse(buf: &mut &[u8]) -> Option<MessageHeader> {
        Some(MessageHeader {
            id: read_u16(buf)?,
            flags: Flags::parse(buf)?,
            qd_count: read_u16(buf)?,
            an_count: read_u16(buf)?,
            ns_count: read_u16(buf)?,
            ar_count: read_u16(buf)?,
        })
    }
}

impl Display for MessageHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{
    ID: {:#x},
    Flags: {},
    Queries: {},
    Answers: {},
    Authoritative answers: {},
    Additional Count: {}
}}",
            self.id,
            indent_string(self.flags.to_string()),
            self.qd_count,
            self.an_count,
            self.ns_count,
            self.ar_count
        )
    }
}
