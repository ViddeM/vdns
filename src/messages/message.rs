use crate::common::formatting::indent_string;
use crate::messages::header::message_header::MessageHeader;
use std::fmt::{Display, Formatter};

pub struct Message {
    header: MessageHeader,
}

impl Message {
    pub fn parse(buf: &mut &[u8]) -> Option<Message> {
        Some(Message {
            header: MessageHeader::parse(buf)?,
        })
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{
    Header: {}
}}",
            indent_string(self.header.to_string())
        )
    }
}
