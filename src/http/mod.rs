mod estatic;

use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
use axum::Extension;
use tower_http::trace::TraceLayer;

use tower::ServiceBuilder;

use super::Configuration;

#[allow(dead_code)]
#[derive(Clone)]
struct Context{
    config: Arc<Configuration>,
}

pub async fn serve(config: Configuration) -> anyhow::Result<()>{
    let app = estatic::router().layer(
        ServiceBuilder::new()
            .layer(Extension(Context {
                config: Arc::new(config.clone()),
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http())
    );
    axum::Server::bind(
        &SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.get_port()))
        .serve(app.into_make_service())
        .await
        .map_err(|_err| anyhow::anyhow!("Can't init"))
}
