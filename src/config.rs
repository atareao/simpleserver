use serde::{Serialize, Deserialize};
use serde_yaml::Error;
use std::process;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration{
    log_level: String,
    port: u16,
}

impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }
    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }

    pub fn get_port(&self) -> u16{
        self.port
    }

    pub async fn read_configuration() -> Self{
        let content = match tokio::fs::read_to_string("config.yml")
            .await {
                Ok(value) => value,
                Err(e) => {
                    println!("Error with config file `config.yml`: {}",
                        e.to_string());
                    process::exit(0);
                }
            };
        match Self::new(&content){
            Ok(configuration) => configuration,
            Err(e) => {
                println!("Error with config file `config.yml`: {}",
                    e.to_string());
                process::exit(0);
            }
        }
    }
}
