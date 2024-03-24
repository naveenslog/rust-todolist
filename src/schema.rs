// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Text,
        completed -> Bool,
    }
}
