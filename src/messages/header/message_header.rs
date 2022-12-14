use crate::messages::header::flags::Flags;
use crate::messages::parsing::read_u16;
use crate::{common::formatting::indent_string, messages::serializing::write_u16};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct MessageHeader {
    pub id: u16,
    pub flags: Flags,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
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

    pub fn serialize(self, buf: &mut Vec<u8>) {
        write_u16(buf, self.id);
        self.flags.serialize(buf);
        write_u16(buf, self.qd_count);
        write_u16(buf, self.an_count);
        write_u16(buf, self.ns_count);
        write_u16(buf, self.ar_count);
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
