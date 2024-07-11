use actix_web::{post, web, HttpResponse, Responder};
use diesel::deserialize::Queryable;
use diesel::RunQueryDsl;
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use serde::{Deserialize, Serialize};

use crate::auth::create_jwt;
use crate::{db::DbPool, models::user::User, schema::users::dsl::*};

#[derive(Deserialize)]
struct LoginInfo {
    email: String,
    password: String,
}

#[derive(Queryable, Serialize, Debug, Clone)]
struct LoginResponse {
    user: UserResponse,
    token: String,
}

#[derive(Serialize, Debug, Clone)]
struct UserResponse {
    id: i32,
    name: String,
    email: String,
    skill_level: i32,
    preferred_climbing_style: Option<String>,
    preferred_gym: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            skill_level: user.skill_level,
            preferred_climbing_style: user.preferred_climbing_style,
            preferred_gym: user.preferred_gym,
        }
    }
}

#[post("/login")]
async fn login(pool: web::Data<DbPool>, info: web::Json<LoginInfo>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let email_address = info.email.clone();
    let pass = &info.password;

    let user = web::block(move || {
        users
            .filter(email.eq(&email_address))
            .first::<User>(&mut conn)
    })
    .await
    .unwrap();

    match user {
        Ok(user) => {
            if user.verify_password(&pass).unwrap() {
                let token = create_jwt(&user.email).unwrap();
                let user_response = UserResponse::from(user);
                return HttpResponse::Ok().json(LoginResponse {
                    user: user_response,
                    token: token,
                });
            } else {
                return HttpResponse::InternalServerError().finish();
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().finish();
        }
    }
}

// Handle authentication logic here
