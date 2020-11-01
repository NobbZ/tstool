use clap::{
    app_from_crate, crate_authors, crate_description, crate_name,
    crate_version, Arg,
};

fn main() {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("prefix")
                .short("p")
                .long("prefix")
                .takes_value(true),
        )
        .get_matches();

    println!("{}", app.value_of("prefix").unwrap());
}
