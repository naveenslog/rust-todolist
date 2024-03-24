use diesel::{Insertable, Queryable, AsChangeset};
use rocket::serde::{Deserialize, Serialize};
use crate::schema::todo;

// This structure represents a record in the `todo` database table.
#[derive(Queryable, Deserialize, Serialize, Debug)]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

// This structure represents the data necessary to create a new todo item.
#[derive(Insertable, Deserialize, Serialize, Debug, AsChangeset)]
#[diesel(table_name = todo)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    pub completed: bool,
}
