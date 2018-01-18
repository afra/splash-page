//!

#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate base64;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

struct AuthUser(String);

pub fn base64_decode(data: &str) -> Option<String> {

    let dec = base64::decode(data).unwrap();
    return Some(String::from_utf8(dec).unwrap());
}

/// Returns true if `key` is a valid API key string.
fn handle_user(key: &str) -> Option<String> {
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

impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthUser, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];

        return match handle_user(key) {
            Some(user) => Outcome::Success(AuthUser(user)),
            None => Outcome::Forward(()),
        };
    }
}

use std::fs::File;
use std::io::prelude::*;

#[post("/api/v1/open", data = "<state>")]
fn set_open(user: AuthUser, state: String) -> String {
    println!("{}", state);

    let mut file = File::create("state.txt").unwrap();
    file.write_all(state.as_bytes()).unwrap();

    return format!("Hello {}", user.0);
}

#[get("/api/v1/open")]
fn get_open() -> String {
    let mut file = File::open("state.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn main() {
    rocket::ignite().mount("/", routes![set_open, get_open]).launch();
}
