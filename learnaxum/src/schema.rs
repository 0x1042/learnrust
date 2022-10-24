// @generated automatically by Diesel CLI.

diesel::table! {
    user_info (id) {
        id -> Integer,
        user_name -> Text,
        user_email -> Text,
        invalid -> Bool,
    }
}
