// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        skill_level -> Int4,
        preferred_climbing_style -> Nullable<Varchar>,
        preferred_gym -> Nullable<Varchar>,
    }
}
