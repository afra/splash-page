//!
//! 

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

extern crate base64;
extern crate pwhash;
extern crate rand;

pub mod database;
pub mod models;
pub mod schema;
mod security;

// Export some database stuff
pub use database::*;


/// Small utility function to do some basic base64 decoding
#[deprecated]
fn base64_decode(data: &str) -> Option<String> {

    let dec = base64::decode(data).unwrap();
    return Some(String::from_utf8(dec).unwrap());
}

/// Returns true if `key` is a valid API key string.
#[deprecated]
pub fn handle_user(key: &str) -> Option<(String, String)> {
    if !key.starts_with("Basic ") {
        return None;
    }

    let dec = base64_decode(&key[6..]).unwrap();
    let split: Vec<_> = dec.split(":").collect();
    return Some((String::from(split[0]), String::from(split[1])));
}
