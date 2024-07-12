// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        sender_id -> Int4,
        receiver_id -> Nullable<Int4>,
        channel_id -> Nullable<Int4>,
        content -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        skill_level -> Int4,
        preferred_climbing_style -> Nullable<Varchar>,
        preferred_gym -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
