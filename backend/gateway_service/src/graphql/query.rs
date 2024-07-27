use crate::graphql::schema::AppContext;
use async_graphql::Context;
use async_graphql::FieldResult;
use log::info;
use shared::models::availability_dto::AvailabilityDTO;
use shared::models::gym_dto::GymDTO;
use shared::models::user_dto::UserDTO;

use super::utils::graphql_request;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn users<'ctx>(&self, ctx: &Context<'ctx>) -> FieldResult<Vec<UserDTO>> {
        let query_string = "{ users { id name email createdAt updatedAt } }";
        let context = ctx.data::<AppContext>()?;

        let response = graphql_request(
            query_string,
            &context.client,
            context.get_user_service_url(),
        )
        .await?;

        let users = response["data"]["users"].clone();
        info!("Users: {:?}", users);
        let users: Vec<UserDTO> = serde_json::from_value(users)?;
        Ok(users)
    }

    async fn user<'ctx>(&self, ctx: &Context<'ctx>, user_id: i32) -> FieldResult<UserDTO> {
        let query_string = format!("{{ user(id: {}) {{ id name email }} }}", user_id);
        let context = ctx.data::<AppContext>()?;

        let response = graphql_request(
            &query_string,
            &context.client,
            context.get_user_service_url(),
        )
        .await?;
        let user = response["data"]["user"].clone();
        let user: UserDTO = serde_json::from_value(user).expect("Error parsing user");
        Ok(user)
    }

    async fn gyms<'ctx>(&self, ctx: &Context<'ctx>) -> FieldResult<Vec<GymDTO>> {
        let query_string = "{ gyms { id name address city state zipCode } }";
        let context = ctx.data::<AppContext>()?;

        let response = graphql_request(
            query_string,
            &context.client,
            context.get_user_service_url(),
        )
        .await?;

        let gyms = response["data"]["gyms"].clone();
        info!("Gyms: {:?}", gyms);
        let gyms: Vec<GymDTO> = serde_json::from_value(gyms)?;
        Ok(gyms)
    }

    async fn availabilities<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: i32,
    ) -> FieldResult<Vec<AvailabilityDTO>> {
        let query_string = format!(
            "{{ availabilities(userId: {}) {{ id gymId startTime endTime }} }}",
            user_id
        );
        let context = ctx.data::<AppContext>()?;

        let response = graphql_request(
            &query_string,
            &context.client,
            context.get_schedule_service_url(),
        )
        .await?;

        let availabilities = response["data"]["availabilities"].clone();
        let availabilities: Vec<AvailabilityDTO> = serde_json::from_value(availabilities)?;
        Ok(availabilities)
    }
}
