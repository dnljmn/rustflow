use std::{ fmt, string};

use config::Config;
use serde::{Deserialize, Serialize};

mod pipeline;
pub use pipeline::Pipeline as Pipeline;

// Define all the configuration here
#[derive(Debug, Serialize, Deserialize)]
pub struct RustflowConfig {
    #[serde(default)]
    pub name: string::String, // Identification of the pipleine
    pub pipelines: Vec<Pipeline>
}

impl fmt::Display for RustflowConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pipelines: String = self.pipelines.iter().map(|x| x.to_string()).collect();
        write!(f, "{}[{}] {}", self.name, self.pipelines.len(), pipelines)
    }
}

// Load config from file
pub fn init_config(file_name: &str) -> RustflowConfig{
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name(file_name))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("RUSTIFY"))
        .build()
        .unwrap();

    let config =  settings
    .try_deserialize::<RustflowConfig>()
    .unwrap();

    config
}