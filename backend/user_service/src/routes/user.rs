use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder};
use shared::models::user_dto::UserDTO;
use std::sync::Arc;

use diesel::query_dsl::methods::FilterDsl;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use futures_util::lock::Mutex;
use log::info;
use serde::Deserialize;

use crate::db::DbPool;
use crate::graphql::schema::Context;
use crate::models::user::{NewUser, User, UserData};
use crate::schema::users;

#[get("/users")]
async fn get_user(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = pool.get().unwrap();
    let all_users: Result<Vec<User>, diesel::result::Error> =
        web::block(move || User::find_all(&mut conn)).await.unwrap();

    if let Ok(all_users) = all_users {
        let cloned_users: Vec<User> = all_users.clone();
        info!("Fetched all users: {:?}", cloned_users);
        HttpResponse::Ok().json(cloned_users)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/users")]
async fn create_user(pool: web::Data<DbPool>, item: web::Json<UserData>) -> impl Responder {
    let mut new_user: NewUser = NewUser::new(item.into_inner());
    new_user.hash_password().expect("Error hashing password");

    let mut conn: PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("Couldn't get connection from pool");

    let created_user: User = web::block(move || User::create(new_user, &mut conn))
        .await
        .expect("Error creating user in blocking thread");

    info!("Created user: {:?}", created_user);
    HttpResponse::Ok().json(created_user)
}

#[derive(Deserialize, Debug)]
struct LoginInfo {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn user_login(
    info: web::Json<LoginInfo>,
    ctx: Data<Arc<Mutex<Context>>>,
) -> impl Responder {
    let mut conn = ctx.lock().await.pool.get().unwrap();
    let email_address = info.email.clone();
    let pass = &info.password;

    let user = web::block(move || {
        users::table
            .filter(users::columns::email.eq(&email_address))
            .first::<User>(&mut conn)
    })
    .await
    .expect("Error loading user");

    info!("User: {:?}", user);
    match user {
        Ok(user) => {
            if user.verify_password(&pass).unwrap() {
                return HttpResponse::Ok().json(UserDTO::from(user));
            } else {
                return HttpResponse::InternalServerError().finish();
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
