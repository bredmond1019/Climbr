use diesel::{PgConnection, RunQueryDsl};

use crate::{models::gym_membership::NewGymMembership, schema::gym_memberships};

pub fn seed_gym_memberships(conn: &mut PgConnection) {
    let test_gym_memberships = vec![
        NewGymMembership::new(1, 1),
        NewGymMembership::new(1, 2),
        NewGymMembership::new(1, 3),
        NewGymMembership::new(1, 4),
        NewGymMembership::new(1, 5),
        NewGymMembership::new(1, 6),
        NewGymMembership::new(1, 7),
        NewGymMembership::new(1, 8),
        NewGymMembership::new(2, 1),
        NewGymMembership::new(2, 2),
        NewGymMembership::new(2, 3),
        NewGymMembership::new(2, 4),
        NewGymMembership::new(2, 5),
        NewGymMembership::new(2, 6),
        NewGymMembership::new(2, 7),
        NewGymMembership::new(2, 8),
        NewGymMembership::new(3, 1),
        NewGymMembership::new(3, 2),
        NewGymMembership::new(3, 3),
        NewGymMembership::new(3, 4),
        NewGymMembership::new(3, 5),
        NewGymMembership::new(3, 6),
        NewGymMembership::new(3, 7),
        NewGymMembership::new(3, 8),
        NewGymMembership::new(4, 1),
        NewGymMembership::new(4, 2),
        NewGymMembership::new(4, 3),
        NewGymMembership::new(4, 4),
        NewGymMembership::new(4, 5),
        NewGymMembership::new(4, 6),
        NewGymMembership::new(4, 7),
        NewGymMembership::new(4, 8),
        NewGymMembership::new(5, 1),
        NewGymMembership::new(5, 2),
        NewGymMembership::new(5, 3),
        NewGymMembership::new(5, 4),
        NewGymMembership::new(5, 5),
    ];

    for gym_membership in test_gym_memberships {
        let gym_memberships_insert = diesel::insert_into(gym_memberships::table);
        gym_memberships_insert
            .values(&gym_membership)
            .execute(conn)
            .expect("Error seeding gym memberships");
    }
}
