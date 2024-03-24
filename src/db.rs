use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;
use rocket_sync_db_pools::{database, diesel};


pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to database: {}", database_url))
}


#[database("sqlite")]
pub struct DbConn(diesel::SqliteConnection);