//!

#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;


#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;


extern crate splash_server as afra;
use afra::database::*;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use std::ops::Deref;

struct AuthUser(String);

// impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
//     type Error = ();

//     fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthUser, ()> {
//         let db = request.guard::<Conn>()?;

//         let keys: Vec<_> = request.headers().get("Authorization").collect();
//         if keys.len() != 1 {
//             return Outcome::Failure((Status::BadRequest, ()));
//         }

//         let key = keys[0];
//         match afra::handle_user(key) {
//             None => return Outcome::Forward(()),
//             Some((user, pw)) => {
//                 let b = afra::check_user_credentials(&*db, &user, &pw);
//                 return match b {
//                     true => Outcome::Success(AuthUser(user)),
//                     false => Outcome::Forward(())
//                 };
//             }
//         }
//     }
// }

use std::fs::File;
use std::io::prelude::*;

use rocket_contrib::Json;


#[derive(Serialize, Deserialize)]
struct NewUserViewModel {
    name: String,
    password: String,
}

#[post("/api/v1/login", data = "<user>")]
fn login(user: Json<NewUserViewModel>) -> String {
    // println!("{}", state);

    // let mut file = File::create("state.txt").unwrap();
    // file.write_all(state.as_bytes()).unwrap();

    return format!("Hello World");
}

// #[post("/api/v1/open", data = "<state>")]
// fn set_open(user: AuthUser, state: String) -> String {
//     println!("{}", state);

//     let mut file = File::create("state.txt").unwrap();
//     file.write_all(state.as_bytes()).unwrap();

//     return format!("Hello {}", user.0);
// }

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
    rocket::ignite()
        .manage(c)
        .mount("/", routes![])
        .launch();
}

////////////////////////////////////////////

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl Deref for Conn {
    type Target = SqliteConnection;

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
