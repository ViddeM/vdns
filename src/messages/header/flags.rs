use crate::messages::parsing::read_u16;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Flags {
    qr: QR,          // 1 bit
    op_code: OpCode, // 4 bits
    aa: bool,        // 1 bit
    tc: bool,        // 1 bit
    rd: bool,        // 1 bit
    ra: bool,        // 1 bit
    z: u8,           // Reserved, must be 0, 1 bit.
    ad: bool,        // 1 bit
    cd: bool,        // 1 bit
    r_code: RCode,   // 4 bits
}

impl Flags {
    pub fn parse(buf: &mut &[u8]) -> Option<Flags> {
        let val = read_u16(buf)?;
        Some(Flags {
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
enum QR {
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
enum OpCode {
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
            OpCode::Query => write!(f, "Query"),
            OpCode::IQuery => write!(f, "IQuery"),
            OpCode::Status => write!(f, "Status"),
            OpCode::Reserved => write!(f, "Reserved"),
        }
    }
}

#[derive(Clone, Debug)]
enum RCode {
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
