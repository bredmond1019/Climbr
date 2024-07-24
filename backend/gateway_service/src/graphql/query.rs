use crate::graphql::{schema::Context, utils::graphql_request};

use juniper::{graphql_object, FieldResult};
use log::info;
use shared::models::{user::User, user_dto::UserDTO};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn users(&self, context: &Context) -> FieldResult<Vec<UserDTO>> {
        let query_string = "{ users { id name email password createdAt updatedAt } }";

        let response = graphql_request(
            query_string,
            &context.client,
            context.get_user_service_url(),
        )
        .await;

        let users = response["data"]["users"].clone();
        info!("Users: {:?}", users);
        let users: Vec<UserDTO> = serde_json::from_value(users).expect("Error parsing users");
        Ok(users)
    }

    async fn user(context: &mut Context, user_id: i32) -> Option<User> {
        let query_string = format!("{{ user(id: {}) {{ id name email }} }}", user_id);
        let response = graphql_request(
            &query_string,
            &context.client,
            context.get_user_service_url(),
        )
        .await;
        let user = response["data"]["user"].clone();
        let user: User = serde_json::from_value(user).expect("Error parsing user");
        Some(user)
    }

    async fn availabilities(context: &Context, user_id: i32) -> FieldResult<Vec<String>> {
        let query_string = format!("{{ availabilities(userId: {}) }}", user_id);
        let response = graphql_request(
            &query_string,
            &context.client,
            context.get_schedule_service_url(),
        )
        .await;

        let availabilities = response["data"]["availabilities"].clone();
        let availabilities: Vec<String> =
            serde_json::from_value(availabilities).expect("Error parsing availabilities");
        Ok(availabilities)
    }
}
