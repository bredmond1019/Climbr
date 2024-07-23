// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    gym_memberships (id) {
        id -> Int4,
        user_id -> Int4,
        gym_id -> Int4,
    }
}

diesel::table! {
    gyms (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(gym_memberships -> gyms (gym_id));
diesel::joinable!(gym_memberships -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(gym_memberships, gyms, users,);

diesel::table! {
    availabilities (id) {
        id -> Int4,
        user_id -> Int4,
        gym_id -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    event_members (id) {
        id -> Int4,
        event_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        gym_id -> Int4,
        requester_id -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(event_members -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(availabilities, event_members, events,);

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

diesel::joinable!(chat_messages -> conversations (conversation_id));
diesel::joinable!(conversation_memberships -> conversations (conversation_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat_messages,
    conversation_memberships,
    conversations,
);
