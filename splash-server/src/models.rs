use std::time::SystemTime;
use schema::*;


#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub pw_hash: String,
    pub salt: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub pw_hash: &'a str,
    pub salt: &'a str,
}

#[derive(Queryable)]
pub struct Session {
    pub id: i32,
    pub token: String,
    pub user: i32,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession<'a> {
    pub token: &'a str,
    pub user: i32,
}

#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub open: bool,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "space_events"]
pub struct NewEvent {
    pub open: bool
}