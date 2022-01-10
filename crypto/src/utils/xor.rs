pub fn xor_simple(data: &mut [u8], key: u32) {
    for char in data {
        unsafe {
            *char = char::from_u32_unchecked(*char as u32 ^ key) as u8;
        }
    }
}

pub fn xor(data: &mut [u8], key: Vec<u8>) {
    for i in 0..data.len() {
        data[i] ^= key[i % key.len()];
    }
}
