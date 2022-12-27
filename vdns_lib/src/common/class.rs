use serde::{Deserialize, Serialize};

use crate::messages::{parsing::Reader, serializing::Writer};
use std::fmt::{Display, Formatter};

use super::parse_error::ParseResult;

#[derive(Serialize, Deserialize)]
pub enum Class {
    Reserved,
    IN, // Internet
    CS, // The CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CH, // Chaos
    HS, // Hesiod
    Unassigned,
}

impl Class {
    pub fn parse(reader: &mut Reader) -> ParseResult<Self> {
        let num = reader.read_u16()?;
        Ok(match num {
            0 => Class::Reserved,
            1 => Class::IN,
            2 => Class::CS,
            3 => Class::CH,
            4 => Class::HS,
            val => {
                println!("Got unassigned CLASS value {val}");
                Class::Unassigned
            }
        })
    }

    pub fn serialize(&self, writer: &mut Writer) {
        let val: u16 = match self {
            Class::Reserved => 0,
            Class::IN => 1,
            Class::CS => 2,
            Class::CH => 3,
            Class::HS => 4,
            Class::Unassigned => 5,
        };

        writer.write_u16(val);
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Class::Reserved => "Reserved",
                Class::IN => "IN (Internet)",
                Class::CS => "CSNET (OBSOLETE!)",
                Class::CH => "Chaos",
                Class::HS => "Hesiod",
                Class::Unassigned => "Unassigned",
            }
        )
    }
}
