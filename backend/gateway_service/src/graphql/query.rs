use crate::graphql::schema::Context;
use crate::graphql::utils::get_graphql_response;

use juniper::{graphql_object, FieldResult};
use log::info;
use shared::models::{user::User, user_dto::UserDTO};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn users(&self, context: &Context) -> FieldResult<Vec<UserDTO>> {
        let query_string = "{ users { id name email password createdAt updatedAt } }";
        let response = get_graphql_response(query_string, context).await;

        let users = response["data"]["users"].clone();
        info!("Users: {:?}", users);
        let users: Vec<UserDTO> = serde_json::from_value(users).expect("Error parsing users");
        Ok(users)
    }

    async fn user(context: &mut Context, user_id: i32) -> Option<User> {
        let query_string = format!("{{ user(id: {}) {{ id name email }} }}", user_id);
        let response = get_graphql_response(&query_string, context).await;

        let user = response["data"]["user"].clone();
        let user: User = serde_json::from_value(user).expect("Error parsing user");
        Some(user)
    }
}
