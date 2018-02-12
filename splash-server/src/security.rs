

use rand::{thread_rng, Rng};
use base64;


pub fn base64_encode(data: &Vec<u8>) -> String {
    return base64::encode(data);
}


/// Random data, encoded as base64 in a string
pub fn generate_salt() -> String {
    let mut vec = Vec::new();

    let mut random_data = [0u8; 1024];
    thread_rng().fill_bytes(&mut random_data);

    for i in 0..1024 {
        vec.push(random_data[i]);
    }

    /* Base64 so we get a nice string */
    return base64_encode(&vec);
}
