use std::fmt::Display;

use crate::{
    common::{domain_name::DomainName, rr_type::RRType},
    messages::parsing::Reader,
};

#[derive(Debug, Clone)]
pub enum RRData {
    CNAME(DomainName),
}

impl RRData {
    pub fn parse(reader: &mut Reader, rr_type: &RRType, length: u16) -> Option<RRData> {
        Some(match rr_type {
            RRType::CNAME => RRData::CNAME(DomainName::parse(reader)?),
            t => todo!("Data for RRType {t} is not yet implemented!"),
        })
    }
}

impl Display for RRData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RRData( {} )",
            match self {
                RRData::CNAME(name) => format!("CNAME( {name} )"),
            }
        )
    }
}
