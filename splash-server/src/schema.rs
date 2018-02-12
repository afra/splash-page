table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        pw_hash -> Binary,
        salt -> Binary,
    }
}
