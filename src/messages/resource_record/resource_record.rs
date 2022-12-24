use std::fmt::Display;

use crate::{
    common::{class::Class, domain_name::DomainName, rr_type::RRType, ttl::TTL},
    messages::parsing::Reader,
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
    pub fn parse(reader: &mut Reader) -> Option<ResourceRecord> {
        let name = DomainName::parse(reader)?;
        let record_type = RRType::parse(reader)?;
        let class = Class::parse(reader)?;
        let ttl = TTL::parse(reader)?;
        let rd_length = reader.read_u16()?;
        let rdata = RRData::parse(reader, &record_type, rd_length)?;

        Some(Self {
            name,
            record_type,
            class,
            ttl,
            rd_length,
            rdata,
        })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        todo!("Eeerh not done");
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
}}
            ",
            self.name, self.record_type, self.class, self.ttl, self.rd_length, self.rdata
        )
    }
}
