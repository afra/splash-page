use diesel;
use diesel::prelude::*;

use r2d2;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;


use dotenv::dotenv;
use std::env;

use pwhash::sha512_crypt;
use security as sec;

use schema::users;
use models::*;


/// Our Sqlite connection pool 💦
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", &database_url);
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("db pool")
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
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
    return sha512_crypt::verify(&combo, &usr.pw_hash);
}