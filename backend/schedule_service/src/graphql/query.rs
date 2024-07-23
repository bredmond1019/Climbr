use crate::graphql::schema::Context;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use juniper::graphql_object;
use log::info;

use chrono::NaiveDate;
use chrono::NaiveTime;
use shared::models::availability::Availability;
use shared::schema::availabilities::{self, columns};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn availabilities(
        context: &Context,
        gym_id: i32,
        date: String,
        start_time: String,
        end_time: String,
    ) -> Vec<Availability> {
        let mut connection = context
            .pool
            .get()
            .expect("Error connecting to the database");
        info!("Fetching availabilities for gym_id: {}", gym_id);
        info!("Date: {}", date);
        info!("Start time: {}", start_time);
        info!("End time: {}", end_time);

        let parsed_date =
            NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Invalid date format");

        // Parse the start and end times
        let parsed_start_time =
            NaiveTime::parse_from_str(&start_time, "%H:%M").expect("Invalid time format");
        let parsed_end_time =
            NaiveTime::parse_from_str(&end_time, "%H:%M").expect("Invalid time format");

        // Combine the date with start and end times
        let start_datetime = parsed_date.and_time(parsed_start_time);
        let end_datetime = parsed_date.and_time(parsed_end_time);

        let current_availabilities = availabilities::table
            .filter(columns::gym_id.eq(gym_id))
            .filter(columns::start_time.le(end_datetime))
            .filter(columns::end_time.ge(start_datetime))
            .load::<Availability>(&mut connection)
            .expect("Error loading availabilities");

        current_availabilities
    }
}
