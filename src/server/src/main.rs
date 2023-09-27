#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { msg: "Hello, Tera Templates!" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![hello, index])
}
