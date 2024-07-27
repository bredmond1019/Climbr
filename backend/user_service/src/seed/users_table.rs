use diesel::{PgConnection, RunQueryDsl};
use shared::schema::users;

use crate::models::user::{NewUser, User, UserData};

pub fn seed_users(conn: &mut PgConnection) {
    let test_users = vec![
        NewUser::new({
            UserData {
                name: "Charlie".to_string(),
                email: "charlie@gmail.com".to_string(),
                password: "password".to_string(),
            }
        }),
        NewUser::new({
            UserData {
                name: "David".to_string(),
                email: "david@yahoo.com".to_string(),
                password: "password".to_string(),
            }
        }),
        NewUser::new({
            UserData {
                name: "Eve".to_string(),
                email: "eve@gmail.com".to_string(),
                password: "password".to_string(),
            }
        }),
        NewUser::new({
            UserData {
                name: "Frank".to_string(),
                email: "frank@yahoo.com".to_string(),
                password: "password".to_string(),
            }
        }),
        NewUser::new({
            UserData {
                name: "Grace".to_string(),
                email: "grace@gmail.com".to_string(),
                password: "password".to_string(),
            }
        }),
        NewUser::new({
            UserData {
                name: "Henry".to_string(),
                email: "henry@yahoo.com".to_string(),
                password: "password".to_string(),
            }
        }),
        NewUser::new({
            UserData {
                name: "Ivy".to_string(),
                email: "ivy@gmail.com".to_string(),
                password: "password".to_string(),
            }
        }),
        NewUser::new({
            UserData {
                name: "Jack".to_string(),
                email: "jack@yahoo.com".to_string(),
                password: "password".to_string(),
            }
        }),
    ];

    for user in test_users {
        User::create(user, conn);
    }
    ()
}
