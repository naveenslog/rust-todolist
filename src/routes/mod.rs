pub mod core;
pub mod todos;
pub mod users;

pub fn get_core_routes() -> Vec<rocket::Route> {
    core::get_core_routes()
}

pub fn get_todo_api_routes() -> Vec<rocket::Route> {
    todos::get_todo_routes()
}

pub fn get_user_api_routes() -> Vec<rocket::Route> {
    users::get_user_routes()
}