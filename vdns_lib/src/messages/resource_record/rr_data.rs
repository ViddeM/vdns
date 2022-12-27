use std::fmt::Display;

use crate::{
    common::{domain_name::DomainName, parse_error::ParseResult, rr_type::RRType},
    messages::{parsing::Reader, serializing::Writer},
};

use super::{a::A, aaaa::AAAA, soa::SOA};

#[derive(Debug, Clone)]
pub enum RRData {
    CNAME(DomainName),
    A(A),
    AAAA(AAAA),
    SOA(SOA),
    TXT(String),
}

impl RRData {
    pub fn parse(reader: &mut Reader, rr_type: &RRType, length: u16) -> ParseResult<RRData> {
        Ok(match rr_type {
            RRType::CNAME => RRData::CNAME(DomainName::parse(reader)?),
            RRType::A => RRData::A(A::parse(reader)?),
            RRType::AAAA => RRData::AAAA(AAAA::parse(reader)?),
            RRType::SOA => RRData::SOA(SOA::parse(reader)?),
            RRType::TXT => RRData::TXT(reader.read_string(length as usize)?),
            t => todo!("Data for RRType {t} is not yet implemented!"),
        })
    }

    pub fn serialize(&self, writer: &mut Writer) {
        match self {
            RRData::CNAME(name) => name.serialize(writer),
            RRData::A(a) => a.serialize(writer),
            RRData::AAAA(aaaa) => aaaa.serialize(writer),
            RRData::SOA(soa) => soa.serialize(writer),
            RRData::TXT(txt) => txt.as_bytes().iter().for_each(|b| writer.write_u8(*b)),
        }
    }
}

impl Display for RRData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RRData::CNAME(name) => format!("CNAME( {name} )"),
                RRData::A(val) => format!("A(Address = {val})"),
                RRData::AAAA(val) => format!("AAAA(Address = {val})"),
                RRData::SOA(val) => format!("SOA({val})"),
                RRData::TXT(val) => format!("TXT('{val}')"),
            }
        )
    }
}
