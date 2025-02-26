// @generated automatically by Diesel CLI.

diesel::table! {
    events (created_at) {
        created_at -> Timestamptz,
        user_id -> Int4,
        kind -> Varchar,
        x_ft -> Float8,
        y_ft -> Float8,
        duration_s -> Float8,
        impressions -> Int4,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    events,
    users,
);
