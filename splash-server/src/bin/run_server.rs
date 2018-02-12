//!

#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

extern crate splash_server as afra;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

struct AuthUser(String);

impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthUser, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];

        return match afra::handle_user(key) {
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
