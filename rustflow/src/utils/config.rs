use std::string;

use config::Config;
use serde::{Deserialize, Serialize};


// Define all the configuration here
#[derive(Debug, Serialize, Deserialize)]
pub struct RustifyConfig {
    #[serde(default)]
    pub name: string::String,
}


// Load config from file
pub fn init_config(file_name: &str) -> RustifyConfig{
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name(file_name))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("RUSTIFY"))
        .build()
        .unwrap();

    let config =  settings
    .try_deserialize::<RustifyConfig>()
    .unwrap();

    config
}


