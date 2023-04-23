mod estatic;

use std::net::{SocketAddr, Ipv4Addr};
use tower_http::trace::TraceLayer;
use std::env::var;

use tower::ServiceBuilder;

pub async fn serve() -> anyhow::Result<()>{
    let port: u16 = var("PORT").ok().and_then(|port| port.parse().ok()).unwrap_or(8080);
    let app = estatic::router().layer(
        ServiceBuilder::new()
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http())
    );
    axum::Server::bind(
        &SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))
        .serve(app.into_make_service())
        .await
        .map_err(|_err| anyhow::anyhow!("Can't init"))
}
