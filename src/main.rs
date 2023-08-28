// main.rs
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Swifties!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
