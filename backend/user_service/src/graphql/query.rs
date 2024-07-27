use crate::{
    graphql::schema::Context,
    models::{gym::Gym, gym_membership::GymMembership, user::User},
    schema::{gym_memberships, gyms},
};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use juniper::{graphql_object, FieldResult};
use log::info;
use shared::{models::user_dto::UserDTO, schema::users};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn users(&self, context: &Context) -> FieldResult<Vec<UserDTO>> {
        let mut connection = context.pool.get()?;
        let results = User::find_all(&mut connection)?;

        let user_dtos: Vec<UserDTO> = results.into_iter().map(UserDTO::from).collect();
        info!("Users: {:?}", user_dtos);
        Ok(user_dtos)
    }

    fn user(context: &mut Context, user_id: i32) -> Option<User> {
        let mut connection = context.pool.get().expect("Error getting db connection");
        let user = users::table
            .filter(users::columns::id.eq(user_id))
            .first::<User>(&mut connection)
            .ok();
        user
    }

    fn gym(context: &Context, gym_id: i32) -> Option<Gym> {
        let mut connection = context.pool.get().expect("Error getting db connection");

        gyms::table
            .filter(gyms::columns::id.eq(gym_id))
            .first(&mut connection)
            .ok()
    }

    fn gym_memberships(context: &Context, member_id: i32) -> Vec<GymMembership> {
        let mut connection = context.pool.get().expect("Error getting db connection");

        gym_memberships::table
            .filter(gym_memberships::columns::user_id.eq(member_id))
            .load(&mut connection)
            .expect("Error loading gym memberships")
    }
}
