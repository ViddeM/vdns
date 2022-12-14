use crate::common::q_class::QClass;
use crate::common::rr_type::RRType;
use crate::messages::parsing::{read_u8, read_vec};
use crate::messages::serializing::write_u8;
use std::fmt::{Display, Formatter};

pub struct Question {
    q_name: Vec<String>,
    q_type: RRType,
    q_class: QClass,
}

impl Question {
    pub fn parse(buf: &mut &[u8]) -> Option<Question> {
        let mut length = read_u8(buf)?;
        let mut domain: Vec<String> = Vec::new();
        while length > 0 {
            let part =
                read_vec(buf, length as usize)?
                    .into_iter()
                    .fold(String::new(), |mut s, u| {
                        s.push(u as char);
                        s
                    });
            domain.push(part);
            length = read_u8(buf)?;
        }

        Some(Question {
            q_name: domain,
            q_type: RRType::parse(buf)?,
            q_class: QClass::parse(buf)?,
        })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        for label in self.q_name.iter() {
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
            self.q_name.join("."),
            self.q_type,
            self.q_class
        )
    }
}
