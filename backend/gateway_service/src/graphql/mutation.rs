use async_graphql::Context;
use async_graphql::FieldResult;
use shared::models::user_dto::UserDTO;

use super::schema::AppContext;
use super::utils::graphql_request;

pub struct Mutation;

#[derive(async_graphql::InputObject)]
struct NewUserInput {
    name: String,
    email: String,
    password: String,
}

#[async_graphql::Object]
impl Mutation {
    async fn create_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: NewUserInput,
    ) -> FieldResult<UserDTO> {
        let context = ctx.data::<AppContext>()?;
        let client = &context.client;
        let user_service_url = context.get_user_service_url();

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

        let response = graphql_request(&query_string, client, user_service_url).await?;

        let user = response["data"]["createUser"].clone();
        let user: UserDTO = serde_json::from_value(user).expect("Error parsing user");
        Ok(user)
    }
}
