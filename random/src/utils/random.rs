use serde_json::Value;

use rand::{prelude::SliceRandom, thread_rng};

pub fn shuffle_array(array: &mut Vec<Value>) {
    let mut rng = thread_rng();
    array.shuffle(&mut rng);
}