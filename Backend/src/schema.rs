// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        user_name -> Varchar,
        user_email -> Varchar,
        user_password -> Varchar,
    }
}
