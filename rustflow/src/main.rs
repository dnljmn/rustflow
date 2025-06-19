use log::info;
use tokio::signal::unix::signal;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::time::Duration;
use tokio::time::sleep;
use tokio::signal::unix::SignalKind;

pub mod utils;

pub const APP_NAME: &'static  str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

const LOG_FILE: &'static str = "log4rs.yml";
const CONFIG_FILE: &'static str = "rustflow.yml";


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Intializing the logger
    utils::init_logger(LOG_FILE); 

     // Initializing the configuration
     let config = utils::init_config(CONFIG_FILE);
    
     info!("Starting {} v{}", APP_NAME, APP_VERSION);
 
     let (tx,  rx) = mpsc::channel(1);
 
     tokio::spawn(async move {
         signal_handler_unix(tx).await;
     });


        // Loading the configuration
    println!("Hello, world!");
    Ok(())
}


// Wait for system signal and return through channel;
async fn signal_handler_unix(tx: Sender<()>) {
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sigint = signal(SignalKind::interrupt()).unwrap();

    loop{
        tokio::select! {
            _ = sigterm.recv() => {
                println!("Received SIGTERM, exiting...");
                let _ = tx.send(()).await; 
            }
            _ = sigint.recv() => {
                println!("Received SIGTERM, exiting...");
                let _ = tx.send(()).await; 
            }
            _ = sleep(Duration::from_millis(1000)) => {
                // No signal received, continue loop
            }
        }
    }
}