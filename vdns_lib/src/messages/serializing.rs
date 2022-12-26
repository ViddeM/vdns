#[inline(always)]
pub fn write_u8(buffer: &mut Vec<u8>, val: u8) {
    buffer.push(val);
}

#[inline(always)]
pub fn write_u16(buffer: &mut Vec<u8>, val: u16) {
    val.to_be_bytes().into_iter().for_each(|b| buffer.push(b));
}

#[inline(always)]
pub fn write_u32(buffer: &mut Vec<u8>, val: u32) {
    val.to_be_bytes().into_iter().for_each(|b| buffer.push(b));
}
