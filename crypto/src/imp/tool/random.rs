use rand::{rngs::SmallRng, Rng, SeedableRng};

pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut rng = SmallRng::from_entropy();
    let mut out = vec![0; len];
    rng.fill(out.as_mut_slice());
    out
}
