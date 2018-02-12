table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        pw_hash -> Text,
        salt -> Text,
    }
}
