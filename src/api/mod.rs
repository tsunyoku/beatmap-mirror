use std::sync::Arc;

use crate::Context;
use anyhow::Context as AnyhowContext;
use axum::{
    body::{Bytes, Full},
    extract::Path,
    http::Response,
    routing, AddExtensionLayer, Json, Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub mod error;
pub mod routes;

use crate::models;
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

pub use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;

async fn serve_swagger_ui(Path(tail): Path<String>) -> Response<Full<Bytes>> {
    let file = utoipa_swagger_ui::serve(&tail[1..], Arc::new(Config::from("/openapi.json")))
        .map_err(|e| anyhow::anyhow!("failed to serve swagger ui: {}", e))
        .unwrap();

    if let Some(file) = file {
        Response::builder()
            .header("content-type", file.content_type)
            .body(Full::from(file.bytes.to_vec()))
            .unwrap()
    } else {
        Response::builder()
            .status(404)
            .body(Full::from("not Found"))
            .unwrap()
    }
}

fn api_router() -> Router {
    #[derive(utoipa::OpenApi)]
    #[openapi(
        paths(
            routes::v1::beatmaps::get_beatmap,
            routes::v1::beatmapsets::get_beatmapset,
            routes::v1::beatmapsets::search_beatmapsets,
            routes::v2::beatmaps::get_beatmap,
            routes::v2::beatmapsets::get_beatmapset,
            routes::v2::beatmapsets::search_beatmapsets,
            routes::downloads::get_beatmapset
        ),
        components(schemas(
            models::cheesegull::beatmap::CheesegullBeatmap,
            models::cheesegull::beatmapset::CheesegullBeatmapset
        )),
        tags(
            (name = "v1", description = "Cheesegull endpoints"),
            (name = "v2", description = "osu!api v2 endpoints"),
            (name = "general", description = "General endpoints")
        )
    )]
    struct ApiDoc;

    Router::new()
        .route(
            // openapi docs
            "/openapi.json",
            routing::get({
                let doc = ApiDoc::openapi();
                move || async { Json(doc) }
            }),
        )
        .route(
            // swagger ui
            "/docs/*tail",
            routing::get(serve_swagger_ui),
        )
        .merge(routes::v1::beatmaps::router())
        .merge(routes::v1::beatmapsets::router())
        .merge(routes::v2::beatmaps::router())
        .merge(routes::v2::beatmapsets::router())
        .merge(routes::downloads::router())
}

pub async fn serve(context: Context) -> anyhow::Result<()> {
    let server_port = context.config.api_port.clone();

    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(context)),
    );

    log::info!("serving api on {}", server_port);
    axum::Server::bind(&format!("127.0.0.1:{}", server_port).parse()?)
        .serve(app.into_make_service())
        .await
        .context("failed to start api")
}
