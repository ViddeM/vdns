use crate::common::parse_error::{ParseError, ParseResult};
use crate::common::rr_type::RRType;
use crate::common::{domain_name::DomainName, q_class::QClass};
use crate::messages::parsing::Reader;
use crate::messages::serializing::Writer;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Question {
    q_name: DomainName,
    q_type: RRType,
    q_class: QClass,
}

impl Question {
    pub fn parse(reader: &mut Reader) -> ParseResult<Question> {
        let name = DomainName::parse(reader)?;

        Ok(Question {
            q_name: name,
            q_type: RRType::parse(reader).or_else(|err| {
                println!("Failed to parse QType {err}");
                Err(ParseError::Question)
            })?,
            q_class: QClass::parse(reader).or_else(|err| {
                println!("Failed to parse QClass {err}");
                Err(ParseError::Question)
            })?,
        })
    }

    pub fn serialize(&self, writer: &mut Writer) {
        self.q_name.serialize(writer);
        self.q_type.serialize(writer);
        self.q_class.serialize(writer);
    }

    pub fn new(name: &str, requested_type: RRType) -> Self {
        Self {
            q_name: DomainName::from_string(name),
            q_type: requested_type,
            q_class: QClass::IN,
        }
    }

    pub fn get_query_name_type(&self) -> (DomainName, RRType) {
        (self.q_name.clone(), self.q_type.clone())
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
