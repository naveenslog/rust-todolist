use diesel::{Insertable, Queryable, AsChangeset};
use rocket::serde::{Deserialize, Serialize};
use chrono;
use crate::schema::user;

#[derive(Queryable, Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize, Debug, AsChangeset)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}
