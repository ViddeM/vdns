use std::fmt::Display;

use crate::messages::{parsing::Reader, serializing::write_u8};

use super::parse_error::{ParseError, ParseResult};

#[derive(Debug, Clone)]
pub struct DomainName {
    pub parts: Vec<String>,
}

const MASK: u8 = 0b0011_1111;
impl DomainName {
    pub fn parse(reader: &mut Reader) -> ParseResult<DomainName> {
        let parts = parse_parts(reader)?;

        Ok(DomainName { parts })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        // TODO: Handle when length is more than 6 bits
        // TODO: Maybe use pointers...
        for part in self.parts.iter() {
            let len = part.len() as u8;
            let truncated = len & MASK;
            if truncated != len {
                panic!("Length of label was too long!");
            }
            write_u8(buf, truncated);
            for b in part.as_bytes() {
                write_u8(buf, *b);
            }
        }
        // End with an empty len
        write_u8(buf, 0);
    }

    pub fn from_string(name: &str) -> Self {
        Self {
            parts: name.split(".").map(|s| s.to_string()).collect(),
        }
    }
}

impl Display for DomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}

fn parse_parts(reader: &mut Reader) -> ParseResult<Vec<String>> {
    let mut parts = vec![];

    loop {
        let oct = reader.read_u8()?;
        if oct == 0 {
            break;
        }

        let remainder = oct & MASK;

        match oct >> 6 {
            0b00 => {
                // Label
                parts.push(parse_label(reader, remainder)?);
            }
            0b11 => {
                // Pointer
                let second_byte = reader.read_u8()?;
                let new_index = (((0u16 | remainder as u16) << 8) | second_byte as u16) as usize;
                let old_index = reader.get_index();
                reader.set_index(new_index);
                parts.append(&mut parse_parts(reader)?);
                reader.set_index(old_index);
                // Always ends after pointer
                return Ok(parts);
            }
            bits => {
                return Err(ParseError::DomainNameError(format!(
                    "Got invalid name part prefix {bits:0b}, expected label (00) or pointer (11) | currently have parts '{}'", parts.join(", ")
                )));
            }
        }
    }

    Ok(parts)
}

fn parse_label(reader: &mut Reader, length: u8) -> ParseResult<String> {
    let bytes = reader.read_vec(length as usize)?;
    Ok(
        String::from_utf8(bytes.clone()).or(Err(ParseError::DomainNameError(format!(
            "Failed to parse domain name part {bytes:?} to utf8 string"
        ))))?,
    )
}
