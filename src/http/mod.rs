pub mod extractor;
pub mod digital_timer;

use axum::{Router, Extension, http};
use axum::routing::get_service;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tower_http::cors::{ CorsLayer, Any };
use http::method::Method;

use crate::config::Config;

#[derive(Clone)]
struct ApiContext {
    config: Config,
    pool: sqlx::SqlitePool,
}

async fn handle_static_server_error(err: std::io::Error) -> (http::StatusCode, String) {
    (http::StatusCode::INTERNAL_SERVER_ERROR, format!("IO error: {}", err))
}

pub async fn serve(config: Config, pool: sqlx::SqlitePool) {
    let ctx = ApiContext { config, pool };
    let serve_dir = get_service(ServeDir::new(ctx.config.static_dir.clone()))
        .handle_error(handle_static_server_error);
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    let address = ctx.config.listen_addr.clone();
    let app = api_router()
        .nest_service("/pages", serve_dir.clone())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(ctx))
                .layer(cors)
                .into_inner(),
        );

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}

fn api_router() -> Router {
    digital_timer::router()
}