pub fn xor_simple(data: &mut [u8], key: u32) {
    for ch in data {
        unsafe {
            *ch = char::from_u32_unchecked(*ch as u32 ^ key) as u8;
        }
    }
}

pub fn xor(data: &mut [u8], key: &[u8]) {
    for i in 0..data.len() {
        data[i] ^= key[i % key.len()];
    }
}
