pub fn xor_simple(data: Vec<u8>, key: u32) -> Vec<u8> {
    unsafe {
        data.iter()
            .map(|&c| c as u32  ^ key)
            .map(|x| char::from_u32_unchecked(x) as u8)
            .collect()
    }
}

pub fn xor(data: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &c)| c ^ key[i%key.len()])
        .collect()
}