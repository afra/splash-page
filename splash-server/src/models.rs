
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub pw_hash: [u8; 64],
    pub salt: [u8; 1024],
}
