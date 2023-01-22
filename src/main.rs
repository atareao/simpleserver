mod http;

use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use std::str::FromStr;
use std::env::var;


#[tokio::main]
async fn main(){
    let log_level: String = var("LOG_LEVEL").unwrap_or("info".to_string());

    tracing_subscriber::registry()
        .with(EnvFilter::from_str(&log_level).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    http::serve().await.unwrap();
}
