// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        fingerprint -> Varchar,
        timezone_offset -> Nullable<Int4>,
        favorite_team -> Nullable<Varchar>,
        dark_mode -> Nullable<Bool>,
    }
}
