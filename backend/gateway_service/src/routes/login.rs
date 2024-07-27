use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, Responder};

use diesel::deserialize::Queryable;
use serde::{Deserialize, Serialize};

use shared::models::user_dto::UserDTO;

use crate::auth::{create_jwt, UserResponse};
use crate::graphql::schema::AppContext;

#[derive(Queryable, Serialize, Debug, Clone, Deserialize)]
struct LoginResponse {
    user: UserResponse,
    token: String,
}

impl From<UserDTO> for UserResponse {
    fn from(user: UserDTO) -> Self {
        UserResponse {
            id: user.id,
            email: user.email.to_string(),
            name: user.name.to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
struct LoginInfo {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn user_login(info: web::Json<LoginInfo>, ctx: Data<AppContext>) -> impl Responder {
    let client = ctx.client.clone();
    let user_service_url = ctx.get_user_service_url();
    let login_info = info.into_inner();

    let response = client
        .post(&format!("{}/login", user_service_url))
        .json(&login_info)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let user: UserDTO = res.json().await.unwrap();
                let token = create_jwt(&user.email).expect("Error creating JWT");

                let login_response = LoginResponse {
                    user: user.into(),
                    token,
                };
                HttpResponse::Ok().json(login_response)
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
