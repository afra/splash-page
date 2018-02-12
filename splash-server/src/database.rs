use diesel;
use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

use pwhash::sha512_crypt;
use security as sec;

use schema::users;
use models::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/// FIXME: Check if a user of that name already exists
pub fn create_user(conn: &SqliteConnection, username: &str, password: &str) -> usize {
    let salt = sec::generate_salt();
    let combo = format!("{}{}", password, salt);
    let hash = sha512_crypt::hash(&combo).unwrap();

    let user = NewUser {
        name: username,
        pw_hash: &hash,
        salt: &salt,
    };

    return diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)
        .expect("Error creating new user!");
}

pub fn list_users(conn: &SqliteConnection) -> Vec<User> {
    use schema::users::dsl::*;
    return users
        .load::<User>(conn)
        .expect("Failed to load users from database!");
}

pub fn check_user_credentials(conn: &SqliteConnection, username: &str, password: &str) -> bool {
    use schema::users::dsl::*;

    let result = users
        .filter(name.eq(username))
        .limit(1)
        .load::<User>(conn)
        .expect("Failed to load user from database!");

    let usr: &User = result.first().unwrap();

    let combo = format!("{}{}", password, usr.salt);
    let new_hash = sha512_crypt::hash(&combo).unwrap();

    return usr.pw_hash == new_hash;
}