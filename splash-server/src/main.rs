//!

#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate base64;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

struct ApiKey(String);

pub fn base64_decode(data: &str) -> Option<String> {

    let dec = base64::decode(data).unwrap();
    return Some(String::from_utf8(dec).unwrap());
}

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str) -> bool {
    // TODO: Get authorised keys somehow
    // key == key[6..];

    // FIXME: Don't panic!

    if !key.starts_with("Basic ") {
        return false;
    }

    let dec = base64_decode(&key[6..]).unwrap();
    let split: Vec<_> = dec.split(":").collect(); 


    return true;
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];
        if is_valid(keys[0]) {
            return Outcome::Forward(())
        }

        return Outcome::Success(ApiKey(key.to_string()));
    }
}

#[get("/sensitive")]
fn sensitive(key: ApiKey) -> &'static str {
    "Sensitive data."
}
#[post("/api/v1/open")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
