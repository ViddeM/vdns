use std::string::FromUtf8Error;

pub struct Reader<'a> {
    buffer: &'a [u8],
    index: usize,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ReaderError {
    #[error("u8 read error")]
    U8,
    #[error("u16 read error")]
    U16,
    #[error("u32 read error")]
    U32,
    #[error("u64 read error")]
    U64,
    #[error("u128 read error")]
    U128,
    #[error("String read error")]
    String,
    #[error("Array read error")]
    Array,
    #[error("Vec read error")]
    Vec,
    #[error("String parse error")]
    StringParse(#[from] FromUtf8Error),
}

pub type ReaderResult<T> = Result<T, ReaderError>;

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

    pub fn read_u8(&mut self) -> ReaderResult<u8> {
        let b = *self.buffer.get(self.index).ok_or(ReaderError::U8)?;
        self.index = self.index + 1;
        Ok(b)
    }

    pub fn read_u16(&mut self) -> ReaderResult<u16> {
        let [b0, b1] = self.read_array().or(Err(ReaderError::U16))?;
        Ok(u16::from_be_bytes([b0, b1]))
    }

    pub fn read_u32(&mut self) -> ReaderResult<u32> {
        let [b0, b1, b2, b3] = self.read_array().or(Err(ReaderError::U32))?;
        Ok(u32::from_be_bytes([b0, b1, b2, b3]))
    }

    pub fn read_u64(&mut self) -> ReaderResult<u64> {
        let [b0, b1, b2, b3, b4, b5, b6, b7] = self.read_array().or(Err(ReaderError::U64))?;
        Ok(u64::from_be_bytes([b0, b1, b2, b3, b4, b5, b6, b7]))
    }

    pub fn read_u128(&mut self) -> ReaderResult<u128> {
        let [b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15] =
            self.read_array()?;
        Ok(u128::from_be_bytes([
            b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15,
        ]))
    }

    pub fn read_string(&mut self, len: usize) -> ReaderResult<String> {
        let bytes = self.read_vec(len).or(Err(ReaderError::String))?;
        Ok(String::from_utf8(bytes)?)
    }

    pub fn read_array<const LEN: usize>(&mut self) -> ReaderResult<[u8; LEN]> {
        let mut arr = [0u8; LEN];
        let end = self.index + LEN;
        let slice = self.buffer.get(self.index..end).ok_or(ReaderError::Array)?;
        arr.copy_from_slice(slice);
        self.index = end;
        Ok(arr)
    }

    pub fn read_vec(&mut self, len: usize) -> ReaderResult<Vec<u8>> {
        let mut to_read = len;
        let readable_length = self.buffer.len() - self.index;
        if len > readable_length {
            to_read = readable_length;
        }

        let end = self.index + to_read;
        let v = Vec::from(self.buffer.get(self.index..end).ok_or(ReaderError::Vec)?);
        self.index = end;
        Ok(v)
    }

    pub fn peek_remaining_bytes(&self) -> &[u8] {
        &self.buffer[self.index..]
    }

    pub fn set_index(&mut self, new_index: usize) {
        self.index = new_index;
    }

    pub fn print_current_message(&self) {
        println!("Buffer as of now: \n{:0x?}\n", &self.buffer[0..self.index]);
        let message = String::from_utf8_lossy(self.buffer);
        println!("Message as of now: \n'{message}'");
    }
}
