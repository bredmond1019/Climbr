use chrono::{DateTime, TimeZone, Utc};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Conversion from User to UserDTO
impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            password: user.password,
            created_at: TimeZone::from_utc_datetime(&Utc, &user.created_at),
            updated_at: TimeZone::from_utc_datetime(&Utc, &user.updated_at),
        }
    }
}
