use actix_web::{post, web, HttpResponse, Responder};
use diesel::deserialize::Queryable;
use diesel::RunQueryDsl;
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use serde::{Deserialize, Serialize};
use shared::models::user::User;
use shared::schema::users;

use crate::auth::create_jwt;
use crate::db::DbPool;

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
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
        }
    }
}

#[post("/login")]
async fn login(pool: web::Data<DbPool>, info: web::Json<LoginInfo>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let email_address = info.email.clone();
    let pass = &info.password;

    let user = web::block(move || {
        users::table
            .filter(users::columns::email.eq(&email_address))
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
