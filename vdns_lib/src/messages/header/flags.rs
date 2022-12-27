use crate::{
    common::parse_error::ParseResult,
    messages::{parsing::Reader, serializing::write_u16},
};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Flags {
    pub qr: QR,          // 1 bit
    pub op_code: OpCode, // 4 bits
    pub aa: bool,        // 1 bit
    pub tc: bool,        // 1 bit
    pub rd: bool,        // 1 bit
    pub ra: bool,        // 1 bit
    pub z: u8,           // Reserved, must be 0, 1 bit.
    pub ad: bool,        // 1 bit
    pub cd: bool,        // 1 bit
    pub r_code: RCode,   // 4 bits
}

impl Flags {
    pub fn parse(reader: &mut Reader) -> ParseResult<Flags> {
        let val = reader.read_u16()?;
        Ok(Flags {
            qr: QR::parse(val),
            op_code: OpCode::parse(val),
            aa: (val >> 10) & 1 == 1,
            tc: (val >> 9) & 1 == 1,
            rd: (val >> 8) & 1 == 1,
            ra: (val >> 7) & 1 == 1,
            z: ((val >> 6) & 1) as u8,
            ad: (val >> 5) & 1 == 1,
            cd: (val >> 4) & 1 == 1,
            r_code: RCode::parse(val),
        })
    }

    pub fn serialize(self, buf: &mut Vec<u8>) {
        let mut first_byte = match self.qr {
            QR::Query => 0u8,
            QR::Response => 1u8,
        } << 7u8;
        first_byte = first_byte
            | match self.op_code {
                OpCode::Query => 0,
                OpCode::IQuery => 1,
                OpCode::Status => 2,
                OpCode::Reserved => 3, // 3-15 is reserved, picked one.
            } << 3u8;
        first_byte = first_byte
            | match (self.aa, self.tc, self.rd) {
                (false, false, false) => 0b000,
                (false, false, true) => 0b001,
                (false, true, false) => 0b010,
                (false, true, true) => 0b011,
                (true, false, false) => 0b100,
                (true, false, true) => 0b101,
                (true, true, false) => 0b110,
                (true, true, true) => 0b111,
            };

        let mut second_byte = if self.ra { 1 } else { 0 } << 7u8;
        // Three zero bits
        second_byte = second_byte
            | match self.r_code {
                RCode::NoError => 0,
                RCode::FormatError => 1,
                RCode::ServerFailure => 2,
                RCode::NameError => 3,
                RCode::NotImplemented => 4,
                RCode::Refused => 5,
                RCode::Reserved => 6, // 6-15 is reserved, picked one.
            };

        write_u16(buf, ((first_byte as u16) << 8) | second_byte as u16);
    }

    pub fn new_query(recurse: bool) -> Self {
        Self {
            qr: QR::Query,
            op_code: OpCode::Query,
            aa: false,
            tc: false,
            rd: recurse,
            ra: false,
            z: 0,
            ad: false,
            cd: false,
            r_code: RCode::NoError,
        }
    }

    pub fn new_response(query_flags: &Flags) -> Self {
        Self {
            qr: QR::Response,
            op_code: query_flags.op_code.clone(),
            aa: false, // TODO: Set this?
            tc: false,
            rd: query_flags.rd,
            ra: true, // We support recursion!
            z: 0,
            ad: false,
            cd: false,
            r_code: RCode::NoError, // Maybe want to set this differently later on...
        }
    }

    pub fn is_query(&self) -> bool {
        match self.qr {
            QR::Query => true,
            QR::Response => false,
        }
    }

    pub fn recurse(&self) -> bool {
        return self.rd;
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{
    QR: {},
    Op Code: {},
    Authoritative: {},
    Truncated: {},
    Recursion Desired: {},
    Recursion Available: {},
    Z: reserved ({}),
    Answer authenticated: {},
    Non-authenticated data: {},
    Reply Code: {}
}}",
            self.qr,
            self.op_code,
            self.aa,
            self.tc,
            self.rd,
            self.ra,
            self.z,
            self.ad,
            self.cd,
            self.r_code
        )
    }
}

#[derive(Clone, Debug)]
pub enum QR {
    Query,
    Response,
}

impl QR {
    fn parse(val: u16) -> QR {
        let is_response = (val >> 15) & 1 == 1;
        if is_response {
            QR::Response
        } else {
            QR::Query
        }
    }
}

impl Display for QR {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QR::Query => write!(f, "Query"),
            QR::Response => write!(f, "Response"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OpCode {
    Query,
    IQuery,
    Status,
    Reserved, // Not used, reserved for future use.
}

impl OpCode {
    fn parse(val: u16) -> OpCode {
        let num = (val >> 11) & 0b1111;
        match num {
            0 => OpCode::Query,
            1 => OpCode::IQuery,
            2 => OpCode::Status,
            _ => OpCode::Reserved,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Query => write!(f, "Standard Query"),
            OpCode::IQuery => write!(f, "Inverse Query"),
            OpCode::Status => write!(f, "Status"),
            OpCode::Reserved => write!(f, "Reserved"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RCode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
    Reserved, // Not used, reserved for future use.
}

impl RCode {
    fn parse(val: u16) -> RCode {
        let num = val & 0b1111;
        match num {
            0 => RCode::NoError,
            1 => RCode::FormatError,
            2 => RCode::ServerFailure,
            3 => RCode::NameError,
            4 => RCode::NotImplemented,
            5 => RCode::Refused,
            _ => RCode::NoError,
        }
    }
}

impl Display for RCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RCode::NoError => write!(f, "No Error"),
            RCode::FormatError => write!(f, "Format Error"),
            RCode::ServerFailure => write!(f, "Server Failure"),
            RCode::NameError => write!(f, "Name Error"),
            RCode::NotImplemented => write!(f, "Not Implemented"),
            RCode::Refused => write!(f, "Refused"),
            RCode::Reserved => write!(f, "Reserved"),
        }
    }
}
