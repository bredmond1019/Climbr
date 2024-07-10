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
    user: User,
    token: String,
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
                return HttpResponse::Ok().json(LoginResponse {
                    user: user,
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
