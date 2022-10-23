use axum::extract::{Extension, Path};
use axum::response::{Headers, IntoResponse};
use axum::{routing::get, Router};

use crate::api::{error, Result};
use crate::{repositories, usecases, Context};

pub fn router() -> Router {
    Router::new().route("/d/:beatmapset_id", get(get_beatmapset))
}

#[utoipa::path(
    get,
    path = "/d/{beatmapset_id}",
    tag = "general",
    responses(
        (status = 200, description = "Found beatmapset successfully", body = Vec<u8>),
        (status = 404, description = "Beatmapset not found")
    ),
    params(
        ("beatmapset_id" = u32, Path, description = "Beatmapset id")
    )
)]
async fn get_beatmapset(
    ctx: Extension<Context>,
    Path(beatmapset_id): Path<u32>,
) -> Result<impl IntoResponse> {
    let beatmapset_option = usecases::beatmapsets::fetch(&ctx, beatmapset_id).await?;
    if beatmapset_option.is_none() {
        return Err(error::Error::NotFound);
    }

    let beatmapset = beatmapset_option.unwrap();
    let beatmapset_name = format!(
        "{} {} - {}",
        beatmapset_id, beatmapset.data.artist, beatmapset.data.title
    );

    let osz_file = repositories::osu::beatmapsets::download(&ctx, beatmapset_id).await?;

    let headers = Headers([
        (
            String::from("Content-Type"),
            String::from("application/octet-stream"),
        ),
        (
            String::from("Content-Disposition"),
            format!("attachment; filename=\"{}.osz\"", beatmapset_name),
        ),
    ]);
    Ok((headers, osz_file))
}
