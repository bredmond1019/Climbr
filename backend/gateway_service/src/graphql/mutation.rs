use juniper::GraphQLObject;
use juniper::{graphql_object, FieldResult};
use serde::Deserialize;
use shared::models::user_dto::UserDTO;

use crate::graphql::schema::Context;

pub struct Mutation;

use juniper::GraphQLInputObject;

use super::utils::graphql_request;

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

#[derive(GraphQLObject, Debug, Deserialize)]
struct LoginResponse {
    user: UserDTO,
    token: String,
}

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_user(context: &Context, params: NewUserInput) -> FieldResult<UserDTO> {
        let query_string = format!(
            "mutation {{
                createUser(input: {{
                    name: \"{}\",
                    email: \"{}\",
                    password: \"{}\"
                }}) {{
                    id
                    name
                    email
                    password
                    createdAt
                    updatedAt
                }}
            }}",
            params.name, params.email, params.password
        );

        let response = graphql_request(
            &query_string,
            &context.client,
            context.get_user_service_url(),
        )
        .await?;

        let user = response["data"]["createUser"].clone();
        let user: UserDTO = serde_json::from_value(user).expect("Error parsing user");
        Ok(user)
    }

    async fn login(context: &Context, params: LoginInput) -> FieldResult<LoginResponse> {
        let query_string = format!(
            "mutation {{
                login(input: {{
                    email: \"{}\",
                    password: \"{}\"
                }}) {{
                    user {{
                        id
                        name
                        email
                        password
                        createdAt
                        updatedAt
                    }}
                    token
                }}
            }}",
            params.email, params.password
        );

        let response = graphql_request(
            &query_string,
            &context.client,
            context.get_user_service_url(),
        )
        .await?;

        let login_response = response["data"]["login"].clone();
        let login_response: LoginResponse =
            serde_json::from_value(login_response).expect("Error parsing login response");
        Ok(login_response)
    }
}
