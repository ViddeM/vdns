use std::fmt::Display;

use crate::{
    common::{
        class::Class, domain_name::DomainName, parse_error::ParseResult, rr_type::RRType, ttl::TTL,
    },
    messages::{
        parsing::Reader,
        serializing::{write_u16, write_u8},
    },
};

use super::rr_data::RRData;

pub struct ResourceRecord {
    name: DomainName,
    record_type: RRType,
    class: Class,
    ttl: TTL,
    rd_length: u16, // Length of the rdata field
    rdata: RRData,
}

impl ResourceRecord {
    pub fn parse(reader: &mut Reader) -> ParseResult<ResourceRecord> {
        let name = DomainName::parse(reader)?;
        let record_type = RRType::parse(reader)?;
        let class = Class::parse(reader)?;
        let ttl = TTL::parse(reader)?;
        let rd_length = reader.read_u16()?;
        let rdata = RRData::parse(reader, &record_type, rd_length)?;

        Ok(Self {
            name,
            record_type,
            class,
            ttl,
            rd_length,
            rdata,
        })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        self.name.serialize(buf);
        self.record_type.serialize(buf);
        self.class.serialize(buf);
        self.ttl.serialize(buf);

        let mut tmp_buf = vec![];
        self.rdata.serialize(&mut tmp_buf);
        write_u16(buf, tmp_buf.len() as u16);

        for b in tmp_buf.into_iter() {
            write_u8(buf, b);
        }
    }
}

impl Display for ResourceRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{
    name: {},
    record_type: {},
    class: {},
    ttl: {},
    data_length: {},
    data: {}
}}",
            self.name, self.record_type, self.class, self.ttl, self.rd_length, self.rdata
        )
    }
}
