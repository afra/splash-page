//!

#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate splash_server as afra;
use afra::database::*;
use afra::models::User;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use rocket_contrib::Json;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use rocket::response::Failure;

use std::ops::Deref;
use std::fs::File;
use std::io::prelude::*;

struct AuthUser(User);
impl Deref for AuthUser {
    type Target = User;
    fn deref(&self) -> &User {
        return &self.0;
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthUser, ()> {
        let db = request.guard::<Conn>()?;

        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];
        return match afra::get_user_with_token(&*db, key.to_string()) {
            Some(user) => Outcome::Success(AuthUser(user)),
            None => Outcome::Forward(()),
        };
    }
}

#[derive(Serialize, Deserialize)]
struct NewUserViewModel {
    name: String,
    password: String,
}

#[post("/api/v1/login", data = "<user>")]
fn login(user: Json<NewUserViewModel>, db: Conn) -> Result<String, Failure> {
    // afra::create_user(&db, &opt.username, &opt.password);
    // println!("{}", state);

    // let mut file = File::create("state.txt").unwrap();
    // file.write_all(state.as_bytes()).unwrap();
    // let user : String = user.name;
    return match afra::maybe_login(&*db, &user.name, &user.password) {
        Some(id) => Ok(format!("{}", id)),
        None => Err(Failure::from(Status::Unauthorized)),
    };
}

#[post("/api/v1/open", data = "<state>")]
fn set_open(user: AuthUser, state: String) -> String {
    println!("{}", state);

    let mut file = File::create("state.txt").unwrap();
    file.write_all(state.as_bytes()).unwrap();

    return format!("Hello {}", user.name);
}

// #[get("/api/v1/open")]
// fn get_open() -> String {
//     println!("Is it open...?");
//     let mut file = File::open("state.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     return contents;
// }

fn main() {
    // Assuming direct control...
    let c = afra::init_pool();
    println!("Ping");
    rocket::ignite()
        .manage(c)
        .mount("/", routes![login, set_open])
        .launch();
}

////////////////////////////////////////////

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Conn {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
