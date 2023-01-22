mod config;
mod http;

use config::Configuration;
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use std::str::FromStr;


#[tokio::main]
async fn main(){
    let configuration = Configuration::read_configuration().await;

    tracing_subscriber::registry()
        .with(EnvFilter::from_str(configuration.get_log_level()).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    http::serve(configuration).await.unwrap();
}
