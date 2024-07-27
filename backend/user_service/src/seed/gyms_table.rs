use diesel::{PgConnection, RunQueryDsl};

use crate::{models::gym::NewGym, schema::gyms};

pub fn seed_gyms(conn: &mut PgConnection) {
    let test_gyms = vec![
        NewGym::new("The Vertical Ascent".to_string()),
        NewGym::new("Boulder Bash".to_string()),
        NewGym::new("Crack Climbers".to_string()),
        NewGym::new("The Dyno Den".to_string()),
        NewGym::new("Slab City".to_string()),
        NewGym::new("The Chimney Challenge".to_string()),
        NewGym::new("Overhang Oasis".to_string()),
        NewGym::new("The Crimp Cave".to_string()),
    ];

    for gym in test_gyms {
        let gym_insert = diesel::insert_into(gyms::table).values(gym);
        gym_insert.execute(conn).expect("Error seeding gyms");
    }
    ()
}
