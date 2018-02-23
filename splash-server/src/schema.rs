table! {
    sessions (id) {
        id -> Int4,
        token -> Varchar,
        user -> Int4,
    }
}

table! {
    space_events (id) {
        id -> Int4,
        open -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        pw_hash -> Varchar,
        salt -> Varchar,
    }
}

joinable!(sessions -> users (user));
