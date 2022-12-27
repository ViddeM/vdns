use std::collections::HashMap;

use crate::common::domain_name::DomainName;

pub struct Writer {
    buffer: Vec<u8>,
    labels: HashMap<String, usize>,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            labels: HashMap::new(),
        }
    }

    pub fn write_u8(&mut self, val: u8) {
        self.buffer.push(val);
    }

    pub fn write_u16(&mut self, val: u16) {
        val.to_be_bytes()
            .into_iter()
            .for_each(|b| self.buffer.push(b));
    }

    pub fn write_u32(&mut self, val: u32) {
        val.to_be_bytes()
            .into_iter()
            .for_each(|b| self.buffer.push(b));
    }

    pub fn get_serialized_message(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Writes the entire buffer of the other writer to this one
    pub fn merge(&mut self, other: &mut Writer) {
        for &b in other.buffer.iter() {
            self.write_u8(b);
        }
    }

    pub fn track_label(&mut self, label: String) {
        self.labels.insert(label, self.buffer.len());
    }

    pub fn lookup_label(&self, label: &String) -> Option<usize> {
        let index = self.labels.get(label)?;
        Some(*index)
    }
}
