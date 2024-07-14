use chrono::{DateTime, Utc};
use diesel::deserialize::Queryable;

#[derive(Queryable)]
#[diesel(table_name = "conversation_members")]
struct ConversationMember {
    id: i32,
    conversation_id: i32,
    user_id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
