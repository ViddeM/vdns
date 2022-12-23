pub struct Reader<'a> {
    buffer: &'a [u8],
    index: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Reader {
            buffer: buf,
            index: 0,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        let b = *self.buffer.get(self.index)?;
        self.index = self.index + 1;
        Some(b)
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let [b0, b1] = self.read_array()?;
        Some(u16::from_be_bytes([b0, b1]))
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        let [b0, b1, b2, b3] = self.read_array()?;
        Some(u32::from_be_bytes([b0, b1, b2, b3]))
    }

    pub fn read_array<const LEN: usize>(&mut self) -> Option<[u8; LEN]> {
        let mut arr = [0u8; LEN];
        let end = self.index + LEN;
        let slice = self.buffer.get(self.index..end)?;
        arr.copy_from_slice(slice);
        self.index = end;
        Some(arr)
    }

    pub fn read_vec(&mut self, len: usize) -> Option<Vec<u8>> {
        let mut to_read = len;
        let readable_length = self.buffer.len() - self.index;
        if len > readable_length {
            to_read = readable_length;
        }

        let end = self.index + to_read;
        let v = Vec::from(self.buffer.get(self.index..end)?);
        self.index = end;
        Some(v)
    }

    pub fn peek_remaining_bytes(&self) -> &[u8] {
        &self.buffer[self.index..]
    }

    pub fn set_index(&mut self, new_index: usize) {
        self.index = new_index;
    }
}
