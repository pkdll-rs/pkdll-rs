use rand::{thread_rng, Rng};

pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut dest: Vec<u8> = vec![0; len];
    thread_rng().fill(dest.as_mut_slice());
    dest
}
