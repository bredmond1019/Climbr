use diesel::prelude::*;
use shared::{establish_connection, models::user_dto::UserDTO, schema::users};

// #[test]
// fn test_connection() {
//     let mut connection = establish_connection();
//     let results = users::table
//         .load::<UserDTO>(&mut connection)
//         .expect("Error loading users");

//     for user in results {
//         println!("User: {:?}", user);
//     }
// }
