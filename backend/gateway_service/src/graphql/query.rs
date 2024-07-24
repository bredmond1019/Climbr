use crate::graphql::schema::Context;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use juniper::{graphql_object, FieldResult};
use shared::{models::user::User, schema::users};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn users(&self, context: &Context) -> FieldResult<Vec<User>> {
        let mut connection = context.pool.get()?;
        let result = User::find_all(&mut connection)?;
        Ok(result)
    }

    fn user(context: &mut Context, user_id: i32) -> Option<User> {
        let mut connection = context.pool.get().expect("Error getting db connection");
        let user = users::table
            .filter(users::columns::id.eq(user_id))
            .first::<User>(&mut connection)
            .ok();
        user
    }
}
