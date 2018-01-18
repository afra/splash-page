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

#[get("/sensitive")]
fn sensitive(user: AuthUser) -> String {
    return format!("Hello {}", user.0);
}

#[post("/api/v1/open")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![sensitive]).launch();
}
