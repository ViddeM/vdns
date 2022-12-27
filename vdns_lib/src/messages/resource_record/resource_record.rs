use std::{fmt::Display, time::Duration};

use serde::{Deserialize, Serialize};

use crate::{
    common::{
        class::Class, domain_name::DomainName, parse_error::ParseResult, rr_type::RRType, ttl::TTL,
    },
    messages::{parsing::Reader, serializing::Writer},
};

use super::rr_data::RRData;

#[derive(Serialize, Deserialize)]
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

    pub fn serialize(&self, writer: &mut Writer) {
        self.name.serialize(writer);
        self.record_type.serialize(writer);
        self.class.serialize(writer);
        self.ttl.serialize(writer);

        let mut inner_writer = Writer::new();
        self.rdata.serialize(&mut inner_writer);

        writer.write_u16(inner_writer.len() as u16);
        writer.merge(&mut inner_writer);
    }

    pub fn get_query_name_type(&self) -> (DomainName, RRType) {
        (self.name.clone(), self.record_type.clone())
    }

    pub fn seconds_until_expiration(&self) -> usize {
        self.ttl.seconds_until_expiration()
    }

    pub fn set_ttl(&mut self, seconds: usize) {
        self.ttl = TTL::Cache(Duration::from_secs(seconds as u64));
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
