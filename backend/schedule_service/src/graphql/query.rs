use async_graphql::{Context, FieldResult};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::info;

use super::schema::AppContext;
use crate::models::availability::Availability;
use crate::schema::availabilities::{self, columns};

pub struct Query;

#[async_graphql::Object]
impl Query {
    pub async fn availabilities<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: i32,
    ) -> FieldResult<Vec<Availability>> {
        let app_context = ctx.data::<AppContext>()?;
        let mut connection = app_context.pool.get().expect("Error getting db connection");

        info!("Fetching availabilities for user_id: {}", user_id);

        let current_availabilities = availabilities::table
            .filter(columns::user_id.eq(user_id))
            .load::<Availability>(&mut connection)
            .expect("Error loading availabilities");

        Ok(current_availabilities)
    }
    // fn availabilities(
    //     context: &Context,
    //     gym_id: i32,
    //     date: String,
    //     start_time: String,
    //     end_time: String,
    // ) -> Vec<Availability> {
    //     let mut connection = context
    //         .pool
    //         .get()
    //         .expect("Error connecting to the database");
    //     info!("Fetching availabilities for gym_id: {}", gym_id);
    //     info!("Date: {}", date);
    //     info!("Start time: {}", start_time);
    //     info!("End time: {}", end_time);

    //     let parsed_date =
    //         NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Invalid date format");

    //     // Parse the start and end times
    //     let parsed_start_time =
    //         NaiveTime::parse_from_str(&start_time, "%H:%M").expect("Invalid time format");
    //     let parsed_end_time =
    //         NaiveTime::parse_from_str(&end_time, "%H:%M").expect("Invalid time format");

    //     // Combine the date with start and end times
    //     let start_datetime = parsed_date.and_time(parsed_start_time);
    //     let end_datetime = parsed_date.and_time(parsed_end_time);

    //     let current_availabilities = availabilities::table
    //         .filter(columns::gym_id.eq(gym_id))
    //         .filter(columns::start_time.le(end_datetime))
    //         .filter(columns::end_time.ge(start_datetime))
    //         .load::<Availability>(&mut connection)
    //         .expect("Error loading availabilities");

    //     current_availabilities
    // }
}
