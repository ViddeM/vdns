use crate::common::formatting::indent_string;
use crate::messages::header::message_header::MessageHeader;
use crate::messages::question::question::Question;
use std::fmt::{Display, Formatter};

use super::{parsing::Reader, resource_record::resource_record::ResourceRecord};

pub struct Message {
    header: MessageHeader,
    questions: Vec<Question>,
    answer: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>,
}

impl Message {
    pub fn parse(buf: &[u8]) -> Option<Message> {
        let mut reader = Reader::new(buf);

        let header = MessageHeader::parse(&mut reader)?;
        let questions = (0..(header.qd_count))
            .map(|_| Some(Question::parse(&mut reader)?))
            .collect::<Option<Vec<Question>>>()?;

        let answer = (0..header.an_count)
            .map(|_| Some(ResourceRecord::parse(&mut reader)?))
            .collect::<Option<Vec<ResourceRecord>>>()?;

        let authority = (0..header.ns_count)
            .map(|_| Some(ResourceRecord::parse(&mut reader)?))
            .collect::<Option<Vec<ResourceRecord>>>()?;

        let additional = (0..header.ar_count)
            .map(|_| Some(ResourceRecord::parse(&mut reader)?))
            .collect::<Option<Vec<ResourceRecord>>>()?;

        Some(Message {
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
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{
    Header: {},
    Questions: [
        {}
    ]
}}",
            indent_string(self.header.to_string()),
            indent_string(indent_string(
                self.questions
                    .iter()
                    .map(|q| q.to_string())
                    .collect::<Vec<String>>()
                    .join(",\n")
            ))
        )
    }
}
