use log::info;
use tokio::sync::mpsc::Receiver;

use crate::utils::config::RustflowConfig;

pub struct Runtime{
    config: RustflowConfig,
}

impl Runtime{
    pub fn new(config: RustflowConfig) -> Self{
        Self{ config }
    }

    // Runtime of the application
    pub async fn run(mut self, mut signal_receiver: Receiver<()>) -> Result<(), Box<dyn std::error::Error>>{
        
        info!("Starting application runtime {}", self.config.name);
       
        loop{
            if signal_receiver.try_recv().is_ok(){
                info!("Signal received, shutting down the runtime of the application.");
                break;
            }

          
            
            //info!("Loop count {} request so far", count);

            // GenerateFlowFile --
        }
        info!("Shutting down the server");
        Ok(())
    }
}