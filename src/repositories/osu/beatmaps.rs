use futures::future;

use rosu_v2::prelude::{Beatmap, OsuError};

use crate::Context;

pub async fn fetch(ctx: &Context, beatmap_id: u32) -> anyhow::Result<Option<Beatmap>, OsuError> {
    let beatmap_result = ctx.osu_api.beatmap().map_id(beatmap_id).await;

    match beatmap_result {
        Ok(beatmap) => Ok(Some(beatmap)),
        Err(OsuError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn bulk_fetch(ctx: &Context, beatmap_ids: Vec<u32>) -> anyhow::Result<Vec<Beatmap>> {
    let beatmap_count = beatmap_ids.len();
    let mut beatmap_futures = Vec::with_capacity(beatmap_count);

    for beatmap_id in beatmap_ids {
        let beatmap_future = fetch(ctx, beatmap_id);
        beatmap_futures.push(beatmap_future);
    }

    let beatmap_future_results = future::join_all(beatmap_futures).await;
    let mut beatmaps: Vec<Beatmap> = Vec::with_capacity(beatmap_count);

    for beatmap_result in beatmap_future_results {
        let beatmap_option = match beatmap_result {
            Ok(beatmap) => beatmap,
            Err(OsuError::NotFound) => None,
            Err(e) => return Err(e.into()),
        };

        if let Some(beatmap) = beatmap_option {
            beatmaps.push(beatmap);
        }
    }

    Ok(beatmaps)
}
