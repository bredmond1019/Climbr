use async_graphql::{Context, FieldResult};
use diesel::RunQueryDsl;

use super::schema::AppContext;
use crate::models::availability::{Availability, NewAvailability};
use crate::models::event::{Event, NewEvent};
use crate::models::event_member::NewEventMember;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_availability<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: i32,
        gym_id: i32,
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
    ) -> FieldResult<Availability> {
        let context = ctx.data::<AppContext>()?;
        let mut connection = context.pool.get()?;

        let new_availability = NewAvailability::new(user_id, gym_id, start_time, end_time);

        let availability = Availability::create(new_availability, &mut connection);

        Ok(availability)
    }

    async fn book_event<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        gym_id: i32,
        requester_id: i32,
        additional_users: Vec<i32>,
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
    ) -> FieldResult<Event> {
        let context = ctx.data::<AppContext>()?;
        let mut connection = context.pool.get()?;

        let new_event = NewEvent::new(gym_id, requester_id, start_time, end_time);

        let event = Event::create(new_event, &mut connection);

        for user_id in additional_users {
            let new_event_member = NewEventMember::new(event.id, user_id);

            diesel::insert_into(crate::schema::event_members::table)
                .values(&new_event_member)
                .execute(&mut connection)
                .expect("Error adding event member");
        }

        Ok(event)
    }
}
