use elasticsearch::SearchParts;

use crate::{
    models::{beatmap::Beatmap, beatmapset::Beatmapset, ranked_status::RankedStatus},
    repositories, Context,
};

async fn update_beatmaps(ctx: &Context) -> anyhow::Result<()> {
    log::info!("starting beatmap update cycle");

    loop {
        let json_query = serde_json::json!({
            "query": {
                "bool": {
                    "filter": [
                        {
                            "terms": {
                                "data.status": [
                                    RankedStatus::Graveyard as u8,
                                    RankedStatus::WorkInProgress as u8,
                                    RankedStatus::Pending as u8,
                                    RankedStatus::Qualified as u8
                                ]
                            }
                        },
                        {
                            "range": { "last_checked": { "lte": "now-1d/d" } }
                        }
                    ]
                }
            }
        });

        let elastic_response = ctx
            .database
            .search(SearchParts::Index(&[&ctx.config.elastic_beatmaps_index]))
            .body(json_query)
            .size(100)
            .send()
            .await?;

        let beatmaps_json: serde_json::Value = elastic_response
            .json::<serde_json::Value>()
            .await?
            .pointer("/hits/hits")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or(serde_json::Value::Array(vec![]));

        let beatmaps: Vec<Beatmap> = beatmaps_json
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                v.pointer("/_source")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
            })
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect();

        for mut beatmap in beatmaps {
            let osu_beatmap = repositories::osu::beatmaps::fetch(ctx, beatmap.data.map_id).await?;

            if let Some(osu_beatmap) = osu_beatmap {
                let now = chrono::Utc::now();
                beatmap.last_checked = now;

                if osu_beatmap != beatmap.data {
                    beatmap.data = osu_beatmap;
                    beatmap.updated_at = now;

                    log::info!("updated beatmap id {}", beatmap.data.map_id);
                }

                repositories::beatmaps::update(ctx, beatmap).await?;
            }
        }
    }
}

async fn update_beatmapsets(ctx: &Context) -> anyhow::Result<()> {
    log::info!("starting beatmapset update cycle");

    loop {
        let json_query = serde_json::json!({
            "query": {
                "bool": {
                    "filter": [
                        {
                            "terms": {
                                "data.status": [
                                    RankedStatus::Graveyard as u8,
                                    RankedStatus::WorkInProgress as u8,
                                    RankedStatus::Pending as u8,
                                    RankedStatus::Qualified as u8
                                ]
                            }
                        },
                        {
                            "range": { "last_checked": { "lte": "now-1d/d" } }
                        }
                    ]
                }
            }
        });

        let elastic_response = ctx
            .database
            .search(SearchParts::Index(&[&ctx.config.elastic_beatmapsets_index]))
            .body(json_query)
            .size(100)
            .send()
            .await?;

        let beatmapsets_json: serde_json::Value = elastic_response
            .json::<serde_json::Value>()
            .await?
            .pointer("/hits/hits")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or(serde_json::Value::Array(vec![]));

        let beatmapsets: Vec<Beatmapset> = beatmapsets_json
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                v.pointer("/_source")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
            })
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect();

        for mut beatmapset in beatmapsets {
            let osu_beatmapset =
                repositories::osu::beatmapsets::fetch(ctx, beatmapset.data.mapset_id).await?;

            if let Some(osu_beatmapset) = osu_beatmapset {
                let now = chrono::Utc::now();
                beatmapset.last_checked = now;

                if osu_beatmapset != beatmapset.data {
                    beatmapset.data = osu_beatmapset;
                    beatmapset.updated_at = now;

                    log::info!("updated beatmapset id {}", beatmapset.data.mapset_id);
                }

                repositories::beatmapsets::update(ctx, beatmapset).await?;
            }
        }
    }
}

pub async fn serve(context: Context) -> anyhow::Result<()> {
    let res = tokio::try_join!(update_beatmaps(&context), update_beatmapsets(&context));
    if !res.is_ok() {
        anyhow::bail!("updater failed: {:?}", res);
    }

    Ok(())
}
