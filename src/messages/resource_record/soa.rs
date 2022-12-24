use std::fmt::Display;

use crate::{common::domain_name::DomainName, messages::parsing::Reader};

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
    pub fn parse(reader: &mut Reader) -> Option<Self> {
        Some(SOA {
            m_name: DomainName::parse(reader)?,
            r_name: DomainName::parse(reader)?,
            serial: reader.read_u32()?,
            refresh: reader.read_u32()?,
            retry: reader.read_u32()?,
            expire: reader.read_u32()?,
            minimum: reader.read_u32()?,
        })
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