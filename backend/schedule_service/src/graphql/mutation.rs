use juniper::graphql_object;

use crate::graphql::schema::Context;
use diesel::RunQueryDsl;
use shared::models::{
    availability::Availability, availability::NewAvailability, event::Event, event::NewEvent,
    event_member::NewEventMember,
};

use shared::schema::{availabilities, event_members, events};

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn create_availability(
        context: &Context,
        user_id: i32,
        gym_id: i32,
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
    ) -> Availability {
        let mut connection = context
            .pool
            .get()
            .expect("Error connecting to the database");

        let new_availability = NewAvailability::new(user_id, gym_id, start_time, end_time);

        diesel::insert_into(availabilities::table)
            .values(&new_availability)
            .get_result(&mut connection)
            .expect("Error creating availability")
    }

    fn book_event(
        context: &Context,
        gym_id: i32,
        requester_id: i32,
        additional_users: Vec<i32>,
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
    ) -> Event {
        let mut connection = context
            .pool
            .get()
            .expect("Error connecting to the database");

        let new_event = NewEvent::new(gym_id, requester_id, start_time, end_time);

        let event: Event = diesel::insert_into(events::table)
            .values(&new_event)
            .get_result(&mut connection)
            .expect("Error booking event");

        for user_id in additional_users {
            let new_event_member = NewEventMember::new(event.id, user_id);

            diesel::insert_into(event_members::table)
                .values(&new_event_member)
                .execute(&mut connection)
                .expect("Error adding event member");
        }

        event
    }
}
