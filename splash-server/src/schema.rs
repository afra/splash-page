table! {
    users (id) {
        id -> Integer,
        name -> Text,
        pw_hash -> Text,
        salt -> Text,
    }
}
