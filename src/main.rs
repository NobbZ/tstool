#![feature(decl_macro)]

use std::collections::HashMap;

use rocket::{get, routes};
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", &HashMap::<String, ()>::new())
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .launch();
}
