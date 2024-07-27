use crate::{
    models::{gym::Gym, gym_membership::GymMembership, user::User},
    schema::{gym_memberships, gyms},
};

use async_graphql::Context;
use async_graphql::FieldResult;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::info;
use shared::{models::user_dto::UserDTO, schema::users};

use super::schema::AppContext;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn users<'ctx>(&self, ctx: &Context<'ctx>) -> FieldResult<Vec<UserDTO>> {
        let app_context = ctx.data::<AppContext>()?;
        let mut connection = app_context.pool.get().expect("Error getting db connection");
        let results = User::find_all(&mut connection)?;

        let user_dtos: Vec<UserDTO> = results.into_iter().map(UserDTO::from).collect();
        info!("Users: {:?}", user_dtos);
        Ok(user_dtos)
    }

    async fn user<'ctx>(&self, ctx: &Context<'ctx>, user_id: i32) -> FieldResult<Option<User>> {
        let app_context = ctx.data::<AppContext>()?;
        let mut connection = app_context.pool.get().expect("Error getting db connection");
        let user = users::table
            .filter(users::columns::id.eq(user_id))
            .first::<User>(&mut connection);
        user
    }

    async fn gym<'ctx>(&self, ctx: &Context<'ctx>, gym_id: i32) -> Option<Gym> {
        let app_context = ctx.data::<AppContext>()?;
        let mut connection = app_context.pool.get().expect("Error getting db connection");

        gyms::table
            .filter(gyms::columns::id.eq(gym_id))
            .first(&mut connection)
            .ok()
    }

    async fn gym_memberships<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        member_id: i32,
    ) -> Vec<GymMembership> {
        let app_context = ctx.data::<AppContext>()?;
        let mut connection = app_context.pool.get().expect("Error getting db connection");

        gym_memberships::table
            .filter(gym_memberships::columns::user_id.eq(member_id))
            .load(&mut connection)
            .expect("Error loading gym memberships")
    }
}
