use crate::{
    api::{error, Result},
    models::cheesegull::beatmap::CheesegullBeatmap,
    usecases, Context,
};
use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};

pub fn router() -> Router {
    Router::new().route("/api/v1/b/:beatmap_id", get(get_beatmap))
}

#[utoipa::path(
    get,
    path = "/api/v1/b/{beatmap_id}",
    tag = "v1",
    responses(
        (status = 200, description = "Found beatmap successfully", body = CheesegullBeatmap),
        (status = 404, description = "Beatmap not found")
    ),
    params(
        ("beatmap_id" = u32, Path, description = "Beatmap id")
    )
)]
async fn get_beatmap(
    ctx: Extension<Context>,
    Path(beatmap_id): Path<u32>,
) -> Result<Json<Option<CheesegullBeatmap>>> {
    let beatmap = usecases::beatmaps::fetch(&ctx, beatmap_id).await?;

    match beatmap {
        Some(beatmap) => Ok(Json(Some(beatmap.data.into()))),
        None => Err(error::Error::NotFound),
    }
}
