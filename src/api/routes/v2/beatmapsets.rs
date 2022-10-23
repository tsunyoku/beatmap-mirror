use crate::{
    api::{error, Result},
    models::{mode::Mode, ranked_status::RankedStatus},
    repositories, usecases, Context,
};
use axum::{
    extract::{Extension, Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use num_traits::FromPrimitive;
use rosu_v2::prelude::Beatmapset as OsuBeatmapset;

pub fn router() -> Router {
    Router::new()
        .route("/api/v2/beatmapsets/:beatmapset_id", get(get_beatmapset))
        .route("/api/v2/beatmapsets/search", get(search_beatmapsets))
}

#[utoipa::path(
    get,
    path = "/api/v2/beatmapsets/{beatmapset_id}",
    tag = "v2",
    responses(
        (status = 200, description = "Found beatmapset successfully"),
        (status = 404, description = "Beatmapset not found")
    ),
    params(
        ("beatmapset_id" = u32, Path, description = "Beatmapset id")
    )
)]
async fn get_beatmapset(
    ctx: Extension<Context>,
    Path(beatmapset_id): Path<u32>,
) -> Result<Json<Option<OsuBeatmapset>>> {
    let beatmapset = usecases::beatmapsets::fetch(&ctx, beatmapset_id).await?;

    match beatmapset {
        Some(beatmapset) => Ok(Json(Some(beatmapset.data))),
        None => Err(error::Error::NotFound),
    }
}

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct SearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osu_direct: Option<bool>,
}

#[utoipa::path(
    get,
    path = "/api/v2/beatmapsets/search",
    tag = "v2",
    params(SearchParams),
    responses(
        (status = 200, description = "Beatmapsets found"),
    ),
)]
async fn search_beatmapsets(
    ctx: Extension<Context>,
    Query(params): Query<SearchParams>,
) -> Result<impl IntoResponse> {
    let beatmapsets = repositories::beatmapsets::search(
        &ctx,
        params.query,
        params.amount.unwrap_or(100),
        params.offset.unwrap_or(0),
        params
            .status
            .map(|s| FromPrimitive::from_i8(s).unwrap_or(RankedStatus::Ranked))
            .unwrap_or(RankedStatus::Ranked),
        params
            .mode
            .map(|s| FromPrimitive::from_i8(s).unwrap_or(Mode::All))
            .unwrap_or(Mode::All),
    )
    .await?;

    let osu_beatmapsets: Vec<OsuBeatmapset> = beatmapsets
        .into_iter()
        .map(|beatmapset| beatmapset.data)
        .collect();

    let direct_response = params.osu_direct.unwrap_or(false);

    Ok(match direct_response {
        true => usecases::beatmapsets::format_to_direct(osu_beatmapsets).into_response(),
        false => Json(osu_beatmapsets).into_response(),
    })
}
