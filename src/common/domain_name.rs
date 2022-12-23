use std::fmt::Display;

use crate::messages::parsing::Reader;

#[derive(Debug, Clone)]
pub struct DomainName {
    parts: Vec<String>,
}

const MASK: u8 = 0b0011_1111;
impl DomainName {
    pub fn parse(reader: &mut Reader) -> Option<DomainName> {
        let mut parts = vec![];

        while let oct = reader.read_u8()? && oct > 0 {
            let remainder = oct & MASK;
            match oct >> 6 {
                0b00 => {
                    // Label
                    parts.push(parse_label(reader, remainder)?);
                }
                0b11 => {
                    // Pointer
                    parts.push(parse_pointer(reader, remainder)?);
                }
                bits => {
                    println!("Invalid name part prefix {bits:0b}");
                    return None
                }
            }
        }

        Some(DomainName { parts })
    }
}

impl Display for DomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}

fn parse_label(reader: &mut Reader, length: u8) -> Option<String> {
    String::from_utf8(reader.read_vec(length as usize)?).ok()
}

fn parse_pointer(reader: &mut Reader, first_byte: u8) -> Option<String> {
    let second_byte = reader.read_u8()?;
    let base = (((0u16 | first_byte as u16) << 8) | second_byte as u16) as usize;
    let length = reader.read_u8_at(base)? as usize;

    String::from_utf8(reader.read_vec_at(base, length)?).ok()
}
