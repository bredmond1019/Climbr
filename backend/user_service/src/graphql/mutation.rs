use diesel::query_dsl::methods::FilterDsl;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use juniper::graphql_value;
use juniper::FieldError;
use juniper::{graphql_object, FieldResult};

use crate::auth::create_jwt;
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
#[derive(GraphQLInputObject)]
struct LoginInput {
    email: String,
    password: String,
}

#[derive(juniper::GraphQLObject)]
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
        };

        new_user.hash_password()?;

        let mut conn = context.pool.get()?;
        let user = User::create(new_user, &mut conn)?;
        Ok(user)
    }
    fn login(context: &Context, params: LoginInput) -> FieldResult<LoginResponse> {
        use crate::schema::users::dsl::*;
        let mut connection = context.pool.get()?;
        let user = users
            .filter(email.eq(&params.email))
            .first::<User>(&mut connection);

        match user {
            Ok(user) => {
                if user.verify_password(&params.password)? {
                    let token = create_jwt(&user.email)?;
                    Ok(LoginResponse { user, token })
                } else {
                    Err(FieldError::new(
                        "Invalid password",
                        graphql_value!({ "field": "password" }),
                    ))
                }
            }
            Err(_) => Err(FieldError::new(
                "User not found",
                graphql_value!({ "field": "email" }),
            )),
        }
    }
}
