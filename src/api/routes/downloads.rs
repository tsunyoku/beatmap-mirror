use axum::extract::{Extension, Path};
use axum::response::{IntoResponse, Redirect};
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

    let osz_url = repositories::osu::beatmapsets::download(&ctx, beatmapset_id).await?;
    match osz_url {
        Some(url) => {
            Ok(Redirect::temporary(url.parse().map_err(|_| {
                anyhow::anyhow!("Failed to parse url: {}", url)
            })?))
        }
        None => Err(error::Error::NotFound),
    }
}
