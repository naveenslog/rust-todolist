use diesel::prelude::*;
use diesel::RunQueryDsl;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::schema::todo::dsl::*;
use crate::models::{NewTodo, Todo};
use crate::db;

pub fn get_core_routes() -> Vec<rocket::Route> {
    routes![hello]
}

pub fn get_api_routes() -> Vec<rocket::Route> {
    routes![get_todos, create_todo, update_todo, delete_todo]
}

#[get("/")]
fn hello() -> String {
    "Hello, World!".to_string()
}

#[get("/todos")]
fn get_todos() -> Result<Json<Vec<Todo>>, String> {
    let conn = &mut db::establish_connection();
    let results = todo.load::<Todo>(conn).expect("Error loading posts");
    Ok(Json(results))
}

#[post("/todos", format = "json", data = "<new_todo>")]
fn create_todo(new_todo: Json<NewTodo>) -> Result<Json<&'static str>, Status> {
    let conn = &mut db::establish_connection();
    let result = diesel::insert_into(todo)
        .values(&new_todo.into_inner())
        .execute(conn);

    match result {
        Ok(_) => Ok(Json("Todo created successfully.")),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[patch("/todos/<pk>", format = "json", data = "<todo_update>")]
fn update_todo(pk: i32, todo_update: Json<NewTodo>) -> Result<Json<&'static str>, Status> {
    let conn = &mut db::establish_connection();
    let instance = todo.filter(id.eq(pk));
    let result = diesel::update(instance)
        .set(todo_update.into_inner())
        .execute(conn);

    match result {
        Ok(0) => Err(Status::NotFound),
        Ok(_) => Ok(Json("Todo updated successfully.")),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/todos/<pk>")]
fn delete_todo(pk: i32) -> Result<Json<&'static str>, Status> {
    let conn = &mut db::establish_connection();
    let result = diesel::delete(todo.filter(id.eq(pk))).execute(conn);

    match result {
        Ok(_) => Ok(Json("Todo deleted successfully.")),
        Err(_) => Err(Status::InternalServerError),
    }
}
