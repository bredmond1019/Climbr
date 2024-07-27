use async_graphql::Context;
use async_graphql::FieldResult;
use shared::models::user_dto::UserDTO;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_user<'ctx>(&self, ctx: &Context<'ctx>) -> FieldResult<UserDTO> {
        let context = ctx.data::<Context>()?;
        let user_service_url = context.get_user_service_url();
        let client = &context.client;

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
}

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
