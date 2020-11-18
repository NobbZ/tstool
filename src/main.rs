#![feature(decl_macro)]

use std::collections::HashMap;

use rocket::{get, routes};
use rocket_contrib::templates::Template;
use slog::{o, Drain};
use slog_async;
use slog_term;

use tstool::database::{self, Tool};

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
            tasks: vec![],
        },
    )
}

#[get("/status")]
fn status() -> Template {
    let deco = slog_term::PlainDecorator::new(std::io::stdout());
    let drain = slog_term::CompactFormat::new(deco).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("version" => "0.5"));

    if let Err(errors) = database::load_from_files(&log, ".") {
        let mut context = HashMap::new();
        context.insert("errors", errors);
        return Template::render("status/errors", context);
    }

    Template::render("status/fine", None as Option<()>)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, tool, status])
        .launch();
}
