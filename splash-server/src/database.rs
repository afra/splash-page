use diesel;
use diesel::prelude::*;

use r2d2;
use diesel::pg::PgConnection;
use diesel::BelongingToDsl;
use r2d2_diesel::ConnectionManager;

use constant_time_eq::constant_time_eq;

use dotenv::dotenv;
use std::env;

use pwhash::sha512_crypt;
use security as sec;

use schema::users;
use models::*;

/// Our Sqlite connection pool 💦
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", &database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("db pool")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
}

/// FIXME: Check if a user of that name already exists
pub fn create_user(conn: &PgConnection, username: &str, password: &str) -> usize {
    let salt = sec::generate_random_string();
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

pub fn list_users(conn: &PgConnection) -> Vec<User> {
    use schema::users::dsl::*;
    return users
        .load::<User>(conn)
        .expect("Failed to load users from database!");
}

/// Get the current state of the hackerspace
pub fn get_current_state(conn: &PgConnection) -> bool {
    use schema::space_events::dsl::*;
    let latest_vec = space_events
        .order(id.desc())
        .limit(1)
        .load::<Event>(conn)
        .expect("Failed to load user from database!");
    let latest = latest_vec.first().unwrap();
    return latest.open;
}

pub fn create_new_event(conn: &PgConnection, state: bool) -> Option<bool> {
    use schema::space_events::dsl::*;

    /* Check what current state is */
    let latest_vec = space_events
        .order(id.desc())
        .limit(1)
        .load::<Event>(conn)
        .expect("Failed to load user from database!");
    let latest = latest_vec.first().unwrap();
    if latest.open {
        return None;
    }

    let event = NewEvent { open: state };
    diesel::insert_into(space_events)
        .values(&event)
        .execute(conn)
        .expect("Error creating new Afra space event!");
    return Some(state);
}

pub fn get_user_with_token(conn: &PgConnection, token: String) -> Option<User> {
    use schema::users::dsl::users;
    use schema::users::dsl::id;
    use schema::sessions::dsl::sessions;
    use schema::sessions::dsl::id as session_id;

    let split = token.split("::").collect::<Vec<&str>>();
    if split.len() != 2 {
        return None;
    }

    let user_id = split[0];
    let token = split[1];
    let i_id = user_id.parse::<i32>().unwrap();

    let sess = sessions
        .filter(session_id.eq(i_id))
        .limit(1)
        .load::<Session>(conn)
        .expect("Failed to load user from database!");
    let session = sess.first().unwrap();

    if !constant_time_eq(token.as_bytes(), &session.token.as_bytes()) {
        return None;
    }

    let u = users
        .filter(id.eq(&session.user))
        .limit(1)
        .load::<User>(conn)
        .expect("Fuck");

    return Some(u.first().unwrap().clone());
}

pub fn maybe_login(conn: &PgConnection, username: &str, password: &str) -> Option<String> {
    use schema::users::dsl::users;
    use schema::users::dsl::name as user_name;
    use schema::sessions::dsl::sessions;
    use schema::sessions::dsl::id as session_id;

    let result = users
        .filter(user_name.eq(username))
        .limit(1)
        .load::<User>(conn)
        .expect("Failed to load user from database!");

    let usr: &User = match result.first() {
        Some(u) => u,
        None => return None,
    };

    let combo = format!("{}{}", password, usr.salt);
    if !sha512_crypt::verify(&combo, &usr.pw_hash) {
        return None;
    }

    let token = sec::generate_random_string();
    let new_session = NewSession {
        token: &token.clone(),
        user: usr.id,
    };

    let id: Vec<i32> = diesel::insert_into(sessions)
        .values(&new_session)
        .returning(session_id)
        .get_results(conn)
        .unwrap();

    let concat = format!("{}::{}", *id.first().unwrap(), token);
    return Some(concat);
}
