table! {
    sessions (id) {
        id -> Integer,
        token -> Text,
        user -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        pw_hash -> Text,
        salt -> Text,
    }
}

joinable!(sessions -> users (user));

allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
