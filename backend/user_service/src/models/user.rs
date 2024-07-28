use crate::schema::users;

use async_graphql::SimpleObject;
use bcrypt::{hash, verify, DEFAULT_COST};

use chrono::{NaiveDateTime, Utc};
use diesel::{
    deserialize::Queryable,
    prelude::Insertable,
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, RunQueryDsl,
};

use log::info;
use serde::{Deserialize, Serialize};
use shared::models::user_dto::UserDTO;

#[derive(Queryable, Serialize, Debug, Clone, SimpleObject, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Conversion from User to UserDTO
impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn new(user: UserData) -> NewUser {
        let now = Utc::now().naive_utc();
        NewUser {
            name: user.name,
            email: user.email,
            password: user.password,
            created_at: now,
            updated_at: now,
        }
    }
    pub fn hash_password(&mut self) -> Result<(), bcrypt::BcryptError> {
        let hashed = hash(self.password.clone(), DEFAULT_COST)?;
        self.password = hashed;
        Ok(())
    }
}

impl User {
    pub fn find_all(
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<User>, diesel::result::Error> {
        let all_users: Result<Vec<User>, diesel::result::Error> = users::table.load(conn);
        info!("{:?}", &all_users);
        all_users
    }

    pub fn create(new_user: NewUser, conn: &mut PgConnection) -> User {
        let user: User = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Error creating user");
        user
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, &self.password)
    }
}
