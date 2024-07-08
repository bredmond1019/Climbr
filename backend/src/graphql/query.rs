use crate::graphql::schema::Context;
use crate::models::user::User;

use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn users(&self, context: &Context) -> FieldResult<Vec<User>> {
        let mut connection = context.pool.get()?;
        let result = User::find_all(&mut connection)?;
        Ok(result)
    }
}
