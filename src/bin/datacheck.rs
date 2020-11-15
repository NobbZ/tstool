use slog::{error, info, o, Drain};
use slog_async;
use slog_term;
use tstool::database;

fn main() -> Result<(), ()> {
    let deco = slog_term::PlainDecorator::new(std::io::stdout());
    let drain = slog_term::CompactFormat::new(deco).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("version" => "0.5"));

    match database::load_from_files(&log, ".") {
        Ok(()) => Ok(info!(log, "Success!")),
        Err(v) => Err(error!(log, "{:?}", v)),
    }
}
