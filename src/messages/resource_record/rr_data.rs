use std::fmt::Display;

use crate::{
    common::{domain_name::DomainName, rr_type::RRType},
    messages::parsing::Reader,
};

use super::{a::A, soa::SOA};

#[derive(Debug, Clone)]
pub enum RRData {
    CNAME(DomainName),
    A(A),
    SOA(SOA),
}

impl RRData {
    pub fn parse(reader: &mut Reader, rr_type: &RRType, length: u16) -> Option<RRData> {
        Some(match rr_type {
            RRType::CNAME => RRData::CNAME(DomainName::parse(reader)?),
            RRType::A => RRData::A(A::parse(reader)?),
            RRType::SOA => RRData::SOA(SOA::parse(reader)?),
            t => todo!("Data for RRType {t} is not yet implemented!"),
        })
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
                RRData::SOA(val) => format!("SOA({val})"),
            }
        )
    }
}
