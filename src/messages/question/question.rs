use crate::common::rr_type::RRType;
use crate::common::{domain_name::DomainName, q_class::QClass};
use crate::messages::parsing::Reader;
use crate::messages::serializing::write_u8;
use std::fmt::{Display, Formatter};

pub struct Question {
    q_name: DomainName,
    q_type: RRType,
    q_class: QClass,
}

impl Question {
    pub fn parse(reader: &mut Reader) -> Option<Question> {
        let name = DomainName::parse(reader)?;

        Some(Question {
            q_name: name,
            q_type: RRType::parse(reader)?,
            q_class: QClass::parse(reader)?,
        })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        for label in self.q_name.parts.iter() {
            let bytes = label.as_bytes();
            write_u8(buf, bytes.len() as u8); // YOLO, shouldn't be more than a byte... Right?
            for &byte in bytes {
                write_u8(buf, byte);
            }
        }
        write_u8(buf, 0); // Write 0 length label to signal that there are no more labels.
        self.q_type.serialize(buf);
        self.q_class.serialize(buf);
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{
    QName: {},
    QType: {},
    QClass: {},
}}
        ",
            self.q_name, self.q_type, self.q_class
        )
    }
}
