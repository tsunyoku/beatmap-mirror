use crate::{
    api::{error, Result},
    usecases, Context,
};
use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use rosu_v2::prelude::Beatmap as OsuBeatmap;

pub fn router() -> Router {
    Router::new().route("/api/v2/beatmaps/:beatmap_id", get(get_beatmap))
}

#[utoipa::path(
    get,
    path = "/api/v2/beatmaps/{beatmap_id}",
    tag = "v2",
    responses(
        (status = 200, description = "Found beatmap successfully"),
        (status = 404, description = "Beatmap not found")
    ),
    params(
        ("beatmap_id" = u32, Path, description = "Beatmap id")
    )
)]
async fn get_beatmap(
    ctx: Extension<Context>,
    Path(beatmap_id): Path<u32>,
) -> Result<Json<Option<OsuBeatmap>>> {
    let beatmap = usecases::beatmaps::fetch(&ctx, beatmap_id).await?;

    match beatmap {
        Some(beatmap) => Ok(Json(Some(beatmap.data))),
        None => Err(error::Error::NotFound),
    }
}
