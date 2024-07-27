use diesel::PgConnection;

mod gym_memberships_table;
mod gyms_table;
mod users_table;

pub fn seed_dev_data(conn: &mut PgConnection) {
    users_table::seed_users(conn);
    gyms_table::seed_gyms(conn);
    gym_memberships_table::seed_gym_memberships(conn);
}
