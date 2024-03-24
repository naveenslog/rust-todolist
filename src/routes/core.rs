
pub fn get_core_routes() -> Vec<rocket::Route> {
    routes![hello]
}

#[get("/")]
pub fn hello() -> String {
    "Hello, World!".to_string()
}
