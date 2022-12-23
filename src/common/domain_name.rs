use std::fmt::Display;

use crate::messages::parsing::Reader;

#[derive(Debug, Clone)]
pub struct DomainName {
    pub parts: Vec<String>,
}

const MASK: u8 = 0b0011_1111;
impl DomainName {
    pub fn parse(reader: &mut Reader) -> Option<DomainName> {
        let parts = parse_parts(reader)?;

        Some(DomainName { parts })
    }
}

impl Display for DomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}

fn parse_parts(reader: &mut Reader) -> Option<Vec<String>> {
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
                return Some(parts);
            }
            bits => {
                println!("Invalid name part prefix {bits:0b}");
                return None;
            }
        }
    }

    Some(parts)
}

fn parse_label(reader: &mut Reader, length: u8) -> Option<String> {
    String::from_utf8(reader.read_vec(length as usize)?).ok()
}
