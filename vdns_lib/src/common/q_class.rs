use crate::messages::{parsing::Reader, serializing::write_u16};
use std::fmt::{Display, Formatter};

use super::parse_error::ParseResult;

#[derive(Debug, Clone)]
pub enum QClass {
    Reserved,
    IN, // Internet
    Unassigned,
    CH, // Chaos
    HS, // Hesiod
    None,
    Any,
    PrivateUse,
}

impl QClass {
    pub fn parse(reader: &mut Reader) -> ParseResult<QClass> {
        let num = reader.read_u16()?;
        Ok(match num {
            0 => QClass::Reserved,
            1 => QClass::IN,
            2 => QClass::Unassigned,
            3 => QClass::CH,
            4 => QClass::HS,
            5..=253 => QClass::Unassigned,
            254 => QClass::None,
            255 => QClass::Any,
            256..=65279 => QClass::Unassigned,
            65280..=65534 => QClass::PrivateUse,
            65535 => QClass::Reserved,
        })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        let val: u16 = match self {
            QClass::Reserved => 0,
            QClass::IN => 1,
            QClass::Unassigned => 2,
            QClass::CH => 3,
            QClass::HS => 4,
            QClass::None => 254,
            QClass::Any => 255,
            QClass::PrivateUse => 65280,
        };

        write_u16(buf, val);
    }
}

impl Display for QClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                QClass::Reserved => "Reserved",
                QClass::IN => "IN (Internet)",
                QClass::Unassigned => "Unassigned",
                QClass::CH => "Chaos",
                QClass::HS => "Hesiod",
                QClass::None => "None",
                QClass::Any => "Any",
                QClass::PrivateUse => "Private Use",
            }
        )
    }
}
