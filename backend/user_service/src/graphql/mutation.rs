use async_graphql::Context;
use async_graphql::FieldResult;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use chrono::Utc;

use crate::models::user::{NewUser, User};

pub struct Mutation;

use juniper::GraphQLInputObject;

use super::schema::AppContext;

#[derive(InputObject)]
struct NewUserInput {
    name: String,
    email: String,
    password: String,
}
#[derive(GraphQLInputObject)]
struct LoginInput {
    email: String,
    password: String,
}

#[derive(SimpleObject, Debug)]
struct LoginResponse {
    user: User,
    token: String,
}

#[async_graphql::Object]
impl Mutation {
    async fn create_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: NewUserInput,
    ) -> FieldResult<User> {
        let now = Utc::now().naive_utc();
        let mut new_user = NewUser {
            name: params.name,
            email: params.email,
            password: params.password,
            created_at: now,
            updated_at: now,
        };

        new_user.hash_password()?;
        let context = ctx.data::<AppContext>()?;

        let mut conn = context.pool.get()?;
        let user = User::create(new_user, &mut conn);
        Ok(user)

        // let user = User {
        //     id: 6,
        //     name: params.name,
        //     email: params.email,
        //     password: params.password,
        //     created_at: chrono::Local::now().naive_local(),
        //     updated_at: chrono::Local::now().naive_local(),
        // };
        // Ok(user)
    }
}
