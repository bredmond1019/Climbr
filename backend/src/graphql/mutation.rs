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
    skill_level: i32,
    preferred_climbing_style: Option<String>,
    preferred_gym: Option<String>,
}

#[graphql_object(context = Context)]
impl Mutation {
    fn create_user(context: &Context, params: NewUserInput) -> FieldResult<User> {
        let mut new_user = NewUser {
            name: params.name,
            email: params.email,
            password: params.password,
            skill_level: params.skill_level,
            preferred_climbing_style: params.preferred_climbing_style,
            preferred_gym: params.preferred_gym,
        };

        new_user.hash_password()?;

        let mut conn = context.pool.get()?;
        let user = User::create(new_user, &mut conn)?;
        Ok(user)
    }
}
