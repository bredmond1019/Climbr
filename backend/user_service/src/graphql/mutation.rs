use juniper::GraphQLObject;
use juniper::{graphql_object, FieldResult};

use crate::graphql::schema::Context;
use crate::models::user::{NewUser, User};

pub struct Mutation;

use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
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

#[derive(GraphQLObject, Debug)]
struct LoginResponse {
    user: User,
    token: String,
}

#[graphql_object(context = Context)]
impl Mutation {
    fn create_user(context: &Context, params: NewUserInput) -> FieldResult<User> {
        let mut new_user = NewUser {
            name: params.name,
            email: params.email,
            password: params.password,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        new_user.hash_password()?;

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
