use clap::ArgMatches;
use config::{Config, ConfigError, Environment};
use serde_derive::Deserialize;

const DATA_PATH_DEFAULT: &str = "/usr/share/tstool";

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub data_path: String,
}

impl Settings {
    pub fn new(args: ArgMatches) -> Result<Self, ConfigError> {
        let mut c = Config::new();

        c.merge(Environment::with_prefix("tstool"))?;

        c.try_into()
    }
}
