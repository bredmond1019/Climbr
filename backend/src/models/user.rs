use crate::schema::users;
use diesel::{
    deserialize::Queryable,
    prelude::Insertable,
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, RunQueryDsl,
};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub skill_level: i32,
    pub preferred_climbing_style: Option<String>,
    pub preferred_gym: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub skill_level: i32,
    pub preferred_climbing_style: Option<String>,
    pub preferred_gym: Option<String>,
}

// impl NewUser<'_> {
//     pub fn hash_password(&mut self) -> Result<(), bcrypt::BcryptError> {
//         let hashed = hash(self.password, DEFAULT_COST)?;
//         self.password = Box::leak(Box::new(hashed));
//         Ok(())
//     }
// }

impl User {
    pub fn find_all(
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<User>, diesel::result::Error> {
        let all_users: Result<Vec<User>, diesel::result::Error> = users::table.load(conn);
        info!("{:?}", &all_users);
        all_users
    }

    pub fn create(
        new_user: NewUser,
        conn: &mut PgConnection,
    ) -> Result<User, diesel::result::Error> {
        let user: Result<User, diesel::result::Error> = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn);
        user
    }
}

// use serde::{Deserialize, Serialize};
// use diesel::prelude::*;
// use super::schema::users;

// #[derive(Queryable, Serialize, Deserialize, Debug)]
// pub struct User {

// }

// #[derive(Insertable, Deserialize)]
// #[table_name = "users"]
// pub struct NewUser<'a> {
//     pub name: &'a str,
//     pub email: &'a str,
//     pub password: &'a str,
//     pub skill_level: i32,
//     pub preferred_climbing_style: Option<&'a str>,
//     pub preferred_gym: Option<&'a str>,
// }
