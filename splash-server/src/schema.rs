table! {
    sessions (id) {
        id -> Int4,
        token -> Varchar,
        user -> Int4,
    }
}

table! {
    space_etas (id) {
        id -> Int4,
        user -> Int4,
        eta -> Timestamp,
        created_at -> Timestamp,
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
        has_key -> Bool,
        pw_hash -> Varchar,
        salt -> Varchar,
    }
}

joinable!(sessions -> users (user));
