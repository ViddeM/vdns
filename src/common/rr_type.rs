use crate::messages::{parsing::read_u16, serializing::write_u16};
use std::fmt::{Display, Formatter};

// Taken from: https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-4
#[derive(Debug, Clone)]
pub enum RRType {
    Zero,
    A,
    NS,
    MD,
    MF,
    CNAME,
    SOA,
    MB,
    MG,
    MR,
    NULL,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
    RP,
    AFSDB,
    X25,
    ISDN,
    RT,
    NSAP,
    NsapPtr,
    SIG,
    KEY,
    PX,
    GPOS,
    AAAA,
    LOC,
    NXT,
    EID,
    NIMLOC,
    SRV,
    ATMA,
    NAPTR,
    KX,
    CERT,
    A6,
    DNAME,
    SINK,
    OPT,
    APL,
    DS,
    SSHFP,
    IPSECKEY,
    RRSIG,
    NSEC,
    DNSKEY,
    DHCID,
    NSEC3,
    NSEC3PARAM,
    TLSA,
    SMIMEA,
    HIP,
    NINFO,
    RKEY,
    TALINK,
    CDS,
    CDNSKEY,
    OPENPGPKEY,
    CSYNC,
    ZONEMD,
    SVCB,
    HTTPS,
    SPF,
    UINFO,
    UID,
    GID,
    UNSPEC,
    NID,
    L32,
    L64,
    LP,
    EUI48,
    EUI64,
    TKEY,
    TSIG,
    IXFR,
    AXFR,
    MAILB,
    MAILA,
    All,
    URI,
    CAA,
    AVC,
    DOA,
    AMTRELAY,
    TA,
    DLV,
    PrivateUse,
    Reserved,
    ReservedFuture,
    ReservedPrivate,
    ReservedStandardsAction,
    Unassigned,
}

impl RRType {
    pub fn parse(buf: &mut &[u8]) -> Option<RRType> {
        let val = read_u16(buf)?;
        Some(match val {
            0 => RRType::Reserved,
            1 => RRType::A,
            2 => RRType::NS,
            3 => RRType::MD,
            4 => RRType::MF,
            5 => RRType::CNAME,
            6 => RRType::SOA,
            7 => RRType::MB,
            8 => RRType::MG,
            9 => RRType::MR,
            10 => RRType::NULL,
            11 => RRType::WKS,
            12 => RRType::PTR,
            13 => RRType::HINFO,
            14 => RRType::MINFO,
            15 => RRType::MX,
            16 => RRType::TXT,
            17 => RRType::RP,
            18 => RRType::AFSDB,
            19 => RRType::X25,
            20 => RRType::ISDN,
            21 => RRType::RT,
            22 => RRType::NSAP,
            23 => RRType::NsapPtr,
            24 => RRType::SIG,
            25 => RRType::KEY,
            26 => RRType::PX,
            27 => RRType::GPOS,
            28 => RRType::AAAA,
            29 => RRType::LOC,
            30 => RRType::NXT,
            31 => RRType::EID,
            32 => RRType::NIMLOC,
            33 => RRType::SRV,
            34 => RRType::ATMA,
            35 => RRType::NAPTR,
            36 => RRType::KX,
            37 => RRType::CERT,
            38 => RRType::A6,
            39 => RRType::DNAME,
            40 => RRType::SINK,
            41 => RRType::OPT,
            42 => RRType::APL,
            43 => RRType::DS,
            44 => RRType::SSHFP,
            45 => RRType::IPSECKEY,
            46 => RRType::RRSIG,
            47 => RRType::NSEC,
            48 => RRType::DNSKEY,
            49 => RRType::DHCID,
            50 => RRType::NSEC3,
            51 => RRType::NSEC3PARAM,
            52 => RRType::TLSA,
            53 => RRType::SMIMEA,
            54 => RRType::Unassigned,
            55 => RRType::HIP,
            56 => RRType::NINFO,
            57 => RRType::RKEY,
            58 => RRType::TALINK,
            59 => RRType::CDS,
            60 => RRType::CDNSKEY,
            61 => RRType::OPENPGPKEY,
            62 => RRType::CSYNC,
            63 => RRType::ZONEMD,
            64 => RRType::SVCB,
            65 => RRType::HTTPS,
            66..=98 => RRType::Unassigned,
            99 => RRType::SPF,
            100 => RRType::UINFO,
            101 => RRType::UID,
            102 => RRType::GID,
            103 => RRType::UNSPEC,
            104 => RRType::NID,
            105 => RRType::L32,
            106 => RRType::L64,
            107 => RRType::LP,
            108 => RRType::EUI48,
            109 => RRType::EUI64,
            110..=248 => RRType::Unassigned,
            249 => RRType::TKEY,
            250 => RRType::TSIG,
            251 => RRType::IXFR,
            252 => RRType::AXFR,
            253 => RRType::MAILB,
            254 => RRType::MAILA,
            255 => RRType::All,
            256 => RRType::URI,
            257 => RRType::CAA,
            258 => RRType::AVC,
            259 => RRType::DOA,
            260 => RRType::AMTRELAY,
            261..=32767 => RRType::Unassigned,
            32768 => RRType::TA,
            32769 => RRType::DLV,
            32770..=65279 => RRType::Unassigned,
            65280..=65534 => RRType::PrivateUse,
            65535 => RRType::Reserved,
        })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        write_u16(
            buf,
            match self {
                RRType::Reserved => 0,
                RRType::A => 1,
                RRType::NS => 2,
                RRType::MD => 3,
                RRType::MF => 4,
                RRType::CNAME => 5,
                RRType::SOA => 6,
                RRType::MB => 7,
                RRType::MG => 8,
                RRType::MR => 9,
                RRType::NULL => 10,
                RRType::WKS => 11,
                RRType::PTR => 12,
                RRType::HINFO => 13,
                RRType::MINFO => 14,
                RRType::MX => 15,
                RRType::TXT => 16,
                RRType::RP => 17,
                RRType::AFSDB => 18,
                RRType::X25 => 19,
                RRType::ISDN => 20,
                RRType::RT => 21,
                RRType::NSAP => 22,
                RRType::NsapPtr => 23,
                RRType::SIG => 24,
                RRType::KEY => 25,
                RRType::PX => 26,
                RRType::GPOS => 27,
                RRType::AAAA => 28,
                RRType::LOC => 29,
                RRType::NXT => 30,
                RRType::EID => 31,
                RRType::NIMLOC => 32,
                RRType::SRV => 33,
                RRType::ATMA => 34,
                RRType::NAPTR => 35,
                RRType::KX => 36,
                RRType::CERT => 37,
                RRType::A6 => 38,
                RRType::DNAME => 39,
                RRType::SINK => 40,
                RRType::OPT => 41,
                RRType::APL => 42,
                RRType::DS => 43,
                RRType::SSHFP => 44,
                RRType::IPSECKEY => 45,
                RRType::RRSIG => 46,
                RRType::NSEC => 47,
                RRType::DNSKEY => 48,
                RRType::DHCID => 49,
                RRType::NSEC3 => 50,
                RRType::NSEC3PARAM => 51,
                RRType::TLSA => 52,
                RRType::SMIMEA => 53,
                RRType::Unassigned => 54,
                RRType::HIP => 55,
                RRType::NINFO => 56,
                RRType::RKEY => 57,
                RRType::TALINK => 58,
                RRType::CDS => 59,
                RRType::CDNSKEY => 60,
                RRType::OPENPGPKEY => 61,
                RRType::CSYNC => 62,
                RRType::ZONEMD => 63,
                RRType::SVCB => 64,
                RRType::HTTPS => 65,
                RRType::Unassigned => 66, // Can be between 66-98, picked one.
                RRType::SPF => 99,
                RRType::UINFO => 100,
                RRType::UID => 101,
                RRType::GID => 102,
                RRType::UNSPEC => 103,
                RRType::NID => 104,
                RRType::L32 => 105,
                RRType::L64 => 106,
                RRType::LP => 107,
                RRType::EUI48 => 108,
                RRType::EUI64 => 109,
                RRType::TKEY => 249,
                RRType::TSIG => 250,
                RRType::IXFR => 251,
                RRType::AXFR => 252,
                RRType::MAILB => 253,
                RRType::MAILA => 254,
                RRType::All => 255,
                RRType::URI => 256,
                RRType::CAA => 257,
                RRType::AVC => 258,
                RRType::DOA => 259,
                RRType::AMTRELAY => 260,
                RRType::TA => 36827,
                RRType::DLV => 36927,
                RRType::Unassigned => 32770, // Can be between 32770-65279, picked one
                RRType::PrivateUse => 65280, // Can be between 65280-65534, picked one
                RRType::Zero => todo!("RRType Zero not implemented"),
                RRType::ReservedFuture => todo!(),
                RRType::ReservedPrivate => todo!(),
                RRType::ReservedStandardsAction => todo!(),
            },
        );
    }
}

impl Display for RRType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
