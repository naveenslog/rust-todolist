// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Text,
        completed -> Bool,
    }
}

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todo,
    user,
);
