#![feature(decl_macro)]

use std::collections::HashMap;

use rocket::{get, routes};
use rocket_contrib::templates::Template;

use tstool::database::Tool;

#[get("/")]
fn index() -> Template {
    Template::render("index", &HashMap::<String, ()>::new())
}

#[get("/tool")]
fn tool() -> Template {
    Template::render(
        "tool/show",
        Tool {
            name: "fnoo".into(),
            id: "".into(),
            itemtype: "".into(),
            quests: vec![],
            skills: vec![],
            tasks: None,
        },
    )
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, tool])
        .launch();
}
