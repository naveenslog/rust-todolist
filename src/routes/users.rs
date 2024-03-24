pub fn get_user_routes() -> Vec<rocket::Route> {
    routes![hello_user,]
}

#[get("/users")]
pub fn hello_user() -> String {
    "Hello, From User Section!".to_string()
}
