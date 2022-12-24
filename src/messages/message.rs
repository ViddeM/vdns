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

        // println!("Parsing header...");
        let header = MessageHeader::parse(&mut reader)?;
        // println!("Parsed header");

        // println!("Parsing {} questions...", header.qd_count);
        let questions = (0..(header.qd_count))
            .map(|_| Some(Question::parse(&mut reader)?))
            .collect::<Option<Vec<Question>>>()?;
        // println!("Parsed questions");

        // println!("Parsing {} answers...", header.an_count);
        let answer = (0..header.an_count)
            .map(|_| Some(ResourceRecord::parse(&mut reader)?))
            .collect::<Option<Vec<ResourceRecord>>>()?;
        // println!("Parsed answers");

        // println!("Parsing {} authorities...", header.ns_count);
        let authority = (0..header.ns_count)
            .map(|_| Some(ResourceRecord::parse(&mut reader)?))
            .collect::<Option<Vec<ResourceRecord>>>()?;
        // println!("Parsed authorities");

        // println!("Parsing {} additionals...", header.ar_count);
        let additional = (0..header.ar_count)
            .map(|_| Some(ResourceRecord::parse(&mut reader)?))
            .collect::<Option<Vec<ResourceRecord>>>()?;
        // println!("Parsed additional");

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
    Questions: [{}],
    Answers: [{}],
    Authority: [{}],
    Additional: [{}]
}}",
            indent_string(self.header.to_string()),
            indent_string(indent_string(
                self.questions
                    .iter()
                    .map(|q| format!("\n{q}"))
                    .collect::<Vec<String>>()
                    .join(",\n")
            )),
            indent_string(indent_string(
                self.answer
                    .iter()
                    .map(|a| format!("\n{a}"))
                    .collect::<Vec<String>>()
                    .join(",\n")
            )),
            indent_string(indent_string(
                self.authority
                    .iter()
                    .map(|a| format!("\n{a}"))
                    .collect::<Vec<String>>()
                    .join(",\n")
            )),
            indent_string(indent_string(
                self.additional
                    .iter()
                    .map(|a| format!("\n{a}"))
                    .collect::<Vec<String>>()
                    .join(",\n")
            )),
        )
    }
}
