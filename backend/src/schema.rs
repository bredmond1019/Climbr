// @generated automatically by Diesel CLI.

diesel::table! {
    chat_messages (id) {
        id -> Int4,
        conversation_id -> Int4,
        user_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    conversation_memberships (id) {
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

diesel::joinable!(chat_messages -> users (user_id));
diesel::joinable!(chat_messages -> conversations (conversation_id));
diesel::joinable!(conversation_memberships -> conversations (conversation_id));
diesel::joinable!(conversation_memberships -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat_messages,
    conversation_memberships,
    conversations,
    users,
);
