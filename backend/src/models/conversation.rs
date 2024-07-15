use diesel::{
    associations::Identifiable, deserialize::Queryable, dsl::count, prelude::Insertable,
    BelongingToDsl, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl, Selectable,
};
use serde::{Deserialize, Serialize};

use crate::schema::{
    conversation_members,
    conversation_members::columns::{conversation_id, id, user_id},
    conversations,
};

use super::conversation_member::ConversationMember;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::conversations)]
pub struct Conversation {
    pub id: i32,
    pub name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = conversations)]
pub struct NewConversation {
    pub name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl NewConversation {
    pub fn new(name: Option<String>) -> Self {
        NewConversation {
            name,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl Conversation {
    pub fn members(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<ConversationMember>, diesel::result::Error> {
        let members = ConversationMember::belonging_to(self).load::<ConversationMember>(conn)?;

        Ok(members)
    }

    pub fn find_existing_conversation(
        conn: &mut PgConnection,
        user1_id: i32,
        user2_id: i32,
    ) -> Result<Option<Conversation>, diesel::result::Error> {
        let existing_conversation_id = conversation_members::table
            .filter(user_id.eq(user1_id))
            .inner_join(conversations::table.on(conversation_id.eq(conversation_id)))
            .filter(user_id.eq(user2_id))
            .select(conversation_id)
            .first::<i32>(conn)?;

        let conversation = conversations::table
            .filter(conversations::columns::id.eq(existing_conversation_id))
            .first::<Conversation>(conn);

        match conversation {
            Ok(conversation) => Ok(Some(conversation)),
            Err(e) => Err(e),
        }
    }

    pub fn find_or_create_conversation(
        conn: &mut PgConnection,
        user_ids: Vec<i32>,
    ) -> Result<Conversation, diesel::result::Error> {
        let user1_id = user_ids[0];
        let user2_id = user_ids[1];
        match Conversation::find_existing_conversation(conn, user1_id, user2_id) {
            Ok(Some(conversation)) => Ok(conversation),
            Ok(None) => {
                let new_conversation = NewConversation::new(None);
                let conversation = diesel::insert_into(conversations::table)
                    .values(&new_conversation)
                    .get_result(conn)?;

                ConversationMember::create(vec![user1_id, user2_id], &conversation, conn)?;

                Ok(conversation)
            }
            Err(e) => Err(e),
        }
    }
}
