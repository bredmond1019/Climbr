use crate::graphql::schema::Context;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use juniper::graphql_object;

use crate::models::availability::Availability;

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn search_availabilities(
        context: &Context,
        climbing_gym_id: i32,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Vec<Availability> {
        use crate::schema::availabilities::dsl::*;
        let mut connection = context
            .pool
            .get()
            .expect("Error connecting to the database");

        let current_availabilities = availabilities
            .filter(gym_id.eq(climbing_gym_id))
            .filter(start_time.le(end))
            .filter(end_time.ge(start))
            .load::<Availability>(&mut connection)
            .expect("Error loading availabilities");

        current_availabilities
    }
}
