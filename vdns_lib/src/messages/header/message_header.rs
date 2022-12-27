use crate::common::parse_error::{ParseError, ParseResult};
use crate::messages::header::flags::Flags;
use crate::messages::parsing::Reader;
use crate::{common::formatting::indent_string, messages::serializing::write_u16};
use rand::Rng;
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
    pub fn parse(reader: &mut Reader) -> ParseResult<MessageHeader> {
        Ok(MessageHeader {
            id: reader.read_u16()?,
            flags: Flags::parse(reader)?,
            qd_count: reader.read_u16()?,
            an_count: reader.read_u16()?,
            ns_count: reader.read_u16()?,
            ar_count: reader.read_u16()?,
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

    pub fn new_query(recurse: bool) -> Self {
        Self {
            id: rand::thread_rng().gen(),
            flags: Flags::new_query(recurse),
            qd_count: 1,
            an_count: 0,
            ns_count: 0,
            ar_count: 0,
        }
    }

    pub fn new_response(
        query: &MessageHeader,
        num_answers: u16,
        num_authoritive_answers: u16,
        num_additionals: u16,
    ) -> Self {
        Self {
            id: query.id,
            flags: Flags::new_response(&query.flags),
            qd_count: query.qd_count,
            an_count: num_answers,
            ns_count: num_authoritive_answers,
            ar_count: num_additionals,
        }
    }

    pub fn is_query(&self) -> bool {
        self.flags.is_query()
    }

    pub fn do_recursion(&self) -> bool {
        self.flags.recurse()
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
