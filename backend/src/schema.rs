// @generated automatically by Diesel CLI.

diesel::table! {
    conversation_members (id) {
        id -> Int4,
        conversation_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    conversations (id) {
        id -> Int4,
        name -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        conversation_id -> Int4,
        sender_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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

diesel::joinable!(conversation_members -> conversations (conversation_id));
diesel::joinable!(messages -> conversations (conversation_id));

diesel::allow_tables_to_appear_in_same_query!(
    conversation_members,
    conversations,
    messages,
    users,
);
