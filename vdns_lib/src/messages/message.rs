use crate::common::domain_name::DomainName;
use crate::common::parse_error::{ParseError, ParseResult};
use crate::common::{formatting::indent_string, rr_type::RRType};
use crate::messages::header::message_header::MessageHeader;
use crate::messages::question::question::Question;
use std::fmt::{Display, Formatter};

use super::{parsing::Reader, resource_record::resource_record::ResourceRecord};

pub struct Message {
    pub header: MessageHeader,
    pub questions: Vec<Question>,
    pub answer: Vec<ResourceRecord>,
    pub authority: Vec<ResourceRecord>,
    pub additional: Vec<ResourceRecord>,
}

impl Message {
    pub fn parse(buf: &[u8]) -> ParseResult<Message> {
        let mut reader = Reader::new(buf);

        let header = MessageHeader::parse(&mut reader)?;

        let questions = (0..(header.qd_count))
            .map(|_| Ok(Question::parse(&mut reader)?))
            .collect::<ParseResult<Vec<Question>>>()
            .or_else(|err| {
                println!("Failed to parse questions: {err}");
                Err(ParseError::Question)
            })?;

        let answer = (0..header.an_count)
            .map(|_| Ok(ResourceRecord::parse(&mut reader)?))
            .collect::<ParseResult<Vec<ResourceRecord>>>()
            .or_else(|err| {
                println!("Failed to parse answer: {err}");
                Err(ParseError::Answer)
            })?;

        let authority = (0..header.ns_count)
            .map(|_| Ok(ResourceRecord::parse(&mut reader)?))
            .collect::<ParseResult<Vec<ResourceRecord>>>()
            .or_else(|err| {
                println!("Failed to parse authorities: {err}");
                Err(ParseError::Authority)
            })?;

        let additional = (0..header.ar_count)
            .map(|_| Ok(ResourceRecord::parse(&mut reader)?))
            .collect::<ParseResult<Vec<ResourceRecord>>>()
            .or_else(|err| {
                println!("Failed to parse additionals: {err}");
                Err(ParseError::Additional)
            })?;

        Ok(Message {
            header,
            questions,
            answer,
            authority,
            additional,
        })
    }

    pub fn serialize<'a>(self, buf: &mut Vec<u8>) {
        self.header.serialize(buf);
        for question in self.questions.iter() {
            question.serialize(buf);
        }
        for answer in self.answer.iter() {
            answer.serialize(buf);
        }
        for authority in self.authority.iter() {
            authority.serialize(buf);
        }
        for additional in self.additional.iter() {
            additional.serialize(buf);
        }
    }

    pub fn new_query(name: &str, record_type: RRType, recurse: bool) -> Self {
        Self {
            header: MessageHeader::new_query(recurse),
            questions: vec![Question::new(name, record_type)],
            answer: vec![],
            authority: vec![],
            additional: vec![],
        }
    }

    pub fn is_query(&self) -> bool {
        self.header.is_query()
    }

    pub fn do_recursion(&self) -> bool {
        self.header.do_recursion()
    }

    pub fn question_names(&self) -> Vec<(DomainName, RRType)> {
        self.questions
            .iter()
            .map(|q| q.get_query_name_type())
            .collect()
    }

    pub fn new_response(query: &Message, answers: Vec<ResourceRecord>) -> Self {
        let authority = vec![];
        let additional = vec![];
        Self {
            header: MessageHeader::new_response(
                &query.header,
                answers.len() as u16,
                authority.len() as u16,
                additional.len() as u16,
            ),
            questions: query.questions.clone(),
            answer: answers,
            authority: authority,
            additional: additional,
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{
    Header: {},
    Questions: {},
    Answers: {},
    Authority: {},
    Additional: {}
}}",
            indent_string(self.header.to_string()),
            if self.questions.is_empty() {
                "[]".to_string()
            } else {
                format!(
                    "[
        {}
    ]",
                    indent_string(indent_string(
                        self.questions
                            .iter()
                            .map(|q| q.to_string())
                            .collect::<Vec<String>>()
                            .join(",\n"),
                    ))
                )
            },
            if self.answer.is_empty() {
                "[]".to_string()
            } else {
                format!(
                    "[
        {}
    ]",
                    indent_string(indent_string(
                        self.answer
                            .iter()
                            .map(|a| a.to_string())
                            .collect::<Vec<String>>()
                            .join(",\n")
                    ))
                )
            },
            if self.authority.is_empty() {
                "[]".to_string()
            } else {
                format!(
                    "[
        {}
    ]",
                    indent_string(indent_string(
                        self.authority
                            .iter()
                            .map(|a| a.to_string())
                            .collect::<Vec<String>>()
                            .join(",\n")
                    ))
                )
            },
            if self.additional.is_empty() {
                "[]".to_string()
            } else {
                format!(
                    "[
        {}
    ]",
                    indent_string(indent_string(
                        self.additional
                            .iter()
                            .map(|a| a.to_string())
                            .collect::<Vec<String>>()
                            .join(",\n")
                    ))
                )
            },
        )
    }
}
