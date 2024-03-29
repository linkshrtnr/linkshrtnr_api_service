use crate::routes;
use anyhow::Result;
use axum::{Extension, Router};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tracing::info;
#[derive(Clone)]
pub struct ApiContext {
    pub db: PgPool,
}

pub async fn serve(db: PgPool) -> Result<(), anyhow::Error> {
    let app = router().layer(ServiceBuilder::new().layer(Extension(ApiContext { db })));

    let port = 3001_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("router initialized, now listening on port {}", port);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
    Ok(())
}

fn router() -> Router {
    Router::new().nest("/", routes::web_routes::get_routes())
}
