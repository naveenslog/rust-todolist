#[macro_use]
extern crate rocket;
extern crate dotenvy;

pub mod routes;
pub mod db;
pub mod models;
pub mod schema;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().expect("Failed to load .env file");
    rocket::build()
        .attach(db::DbConn::fairing())
        .mount("/", routes::get_core_routes())
        .mount("/api", routes::get_user_api_routes())
        .mount("/api", routes::get_todo_api_routes())
}
