//!
//! 

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate base64;
extern crate pwhash;
extern crate rand;


mod database;
mod models;
mod schema;
mod security;

pub use database::{create_user, establish_connection};


/// Small utility function to do some basic base64 decoding
#[deprecated]
fn base64_decode(data: &str) -> Option<String> {

    let dec = base64::decode(data).unwrap();
    return Some(String::from_utf8(dec).unwrap());
}

/// Returns true if `key` is a valid API key string.
#[deprecated]
pub fn handle_user(key: &str) -> Option<String> {
    // TODO: Get authorised keys somehow
    // key == key[6..];

    // FIXME: Don't panic!

    if !key.starts_with("Basic ") {
        return None;
    }

    let dec = base64_decode(&key[6..]).unwrap();
    let split: Vec<_> = dec.split(":").collect();

    let (user, pw) = (split[0], split[1]);

    return match (user, pw) {
        ("afra", "1234") => Some(String::from(user)),
        _ => None,
    };
}
