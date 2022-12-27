use std::fmt::Display;

use crate::{
    common::{domain_name::DomainName, parse_error::ParseResult},
    messages::{parsing::Reader, serializing::write_u32},
};

#[derive(Debug, Clone)]
pub struct SOA {
    m_name: DomainName,
    r_name: DomainName,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    minimum: u32,
}

impl SOA {
    pub fn parse(reader: &mut Reader) -> ParseResult<Self> {
        Ok(SOA {
            m_name: DomainName::parse(reader)?,
            r_name: DomainName::parse(reader)?,
            serial: reader.read_u32()?,
            refresh: reader.read_u32()?,
            retry: reader.read_u32()?,
            expire: reader.read_u32()?,
            minimum: reader.read_u32()?,
        })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        self.m_name.serialize(buf);
        self.r_name.serialize(buf);
        write_u32(buf, self.serial);
        write_u32(buf, self.refresh);
        write_u32(buf, self.retry);
        write_u32(buf, self.expire);
        write_u32(buf, self.minimum);
    }
}

impl Display for SOA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SOA {{
          m_name: {},
          r_name: {},
          serial: {}s,
          refresh: {}s,
          retry: {}s,
          expire: {}s,
          minimum: {}s  
        }}",
            self.m_name,
            self.r_name,
            self.serial,
            self.refresh,
            self.retry,
            self.expire,
            self.minimum
        )
    }
}
