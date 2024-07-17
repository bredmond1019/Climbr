use crate::schema::users;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::{
    deserialize::Queryable,
    prelude::Insertable,
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, RunQueryDsl,
};
use juniper::GraphQLObject;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug, Clone, GraphQLObject)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl NewUser {
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

    pub fn create(
        new_user: NewUser,
        conn: &mut PgConnection,
    ) -> Result<User, diesel::result::Error> {
        let user: Result<User, diesel::result::Error> = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn);
        user
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, &self.password)
    }
}
