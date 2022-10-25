use crate::{
    helpers::elastic,
    models::{beatmap::Beatmap, beatmapset::Beatmapset},
    repositories, Context,
};
use elasticsearch::SearchParts;
use elasticsearch_dsl::{Aggregation, Query, Search};
use std::{sync::Arc, time::Duration};

async fn crawl_beatmaps(ctx: &Context) -> anyhow::Result<()> {
    elastic::create_index_if_not_exists(&ctx.database, &ctx.config.elastic_beatmaps_index).await?;

    let query = Search::new()
        .query(Query::bool().must(Query::term("crawled", true)))
        .aggregate("max_id", Aggregation::max("data.id"));

    let elastic_response = ctx
        .database
        .search(SearchParts::Index(&[&ctx.config.elastic_beatmaps_index]))
        .body(query)
        .send()
        .await?;

    let mut current_id = elastic_response
        .json::<serde_json::Value>()
        .await?
        .pointer("/aggregations/max_id/value")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0) as u64
        + 1;

    log::info!("starting beatmap crawl from id {}", current_id);

    let mut backoff_time = ctx.config.backoff_start;

    loop {
        let beatmap_ids: Vec<u32> = (0..50)
            .collect::<Vec<u32>>()
            .iter()
            .map(|i| current_id as u32 + 1 + i)
            .collect();

        current_id += 50;

        let mut osu_beatmaps =
            match repositories::osu::beatmaps::bulk_fetch(&ctx, beatmap_ids).await {
                Ok(beatmaps) => beatmaps,
                Err(_) => {
                    log::error!("error while fetching beatmaps from id {}", current_id);
                    vec![]
                }
            };

        let beatmaps_found = osu_beatmaps.len();

        log::info!("found {} beatmaps", beatmaps_found);

        if beatmaps_found > 0 {
            backoff_time = ctx.config.backoff_start;

            let now = chrono::Utc::now();
            let beatmaps = osu_beatmaps
                .iter_mut()
                .map(|b| Beatmap {
                    data: b.to_owned(),
                    created_at: now,
                    updated_at: now,
                    last_checked: now,
                    crawled: true,
                })
                .collect();

            repositories::beatmaps::bulk_create(&ctx, beatmaps).await?;
        } else {
            if backoff_time < ctx.config.max_backoff {
                backoff_time = backoff_time.powf(2_f64).min(ctx.config.max_backoff);
            }

            log::warn!(
                "backing off on beatmaps for {} seconds from id {}",
                backoff_time,
                current_id
            );
            tokio::time::sleep(Duration::from_secs(backoff_time as u64)).await;
        }
    }
}

async fn crawl_beatmapsets(ctx: &Context) -> anyhow::Result<()> {
    elastic::create_index_if_not_exists(&ctx.database, &ctx.config.elastic_beatmapsets_index)
        .await?;

    let query = Search::new()
        .query(Query::bool().must(Query::term("crawled", true)))
        .aggregate("max_id", Aggregation::max("data.id"));

    let elastic_response = ctx
        .database
        .search(SearchParts::Index(&[&ctx.config.elastic_beatmapsets_index]))
        .body(query)
        .send()
        .await?;

    let mut current_id = elastic_response
        .json::<serde_json::Value>()
        .await?
        .pointer("/aggregations/max_id/value")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as u32
        + 1;

    log::info!("starting beatmapset crawl from id {}", current_id);

    let mut backoff_time: f64 = ctx.config.backoff_start;

    loop {
        let beatmapset = match repositories::osu::beatmapsets::fetch(&ctx, current_id).await {
            Ok(beatmapset) => beatmapset,
            Err(_) => {
                log::error!("error while fetching beatmapset {}", current_id);
                None
            }
        };
        current_id += 1;

        if let Some(osu_beatmapset) = beatmapset {
            backoff_time = ctx.config.backoff_start;

            let current_time = chrono::Utc::now();
            let beatmapset = Beatmapset {
                data: osu_beatmapset,
                created_at: current_time,
                updated_at: current_time,
                last_checked: current_time,
                crawled: true,
            };

            repositories::beatmapsets::create(&ctx, beatmapset).await?;
            log::info!("indexed beatmapset {}", current_id - 1);
        } else {
            if backoff_time < ctx.config.max_backoff {
                backoff_time = backoff_time.powf(2_f64).min(ctx.config.max_backoff);
            }

            log::warn!(
                "backing off on beatmapsets for {} seconds from id {}",
                backoff_time,
                current_id
            );
            tokio::time::sleep(Duration::from_secs(backoff_time as u64)).await;
        }
    }
}

pub async fn serve(context: Context) -> anyhow::Result<()> {
    let context_arc = Arc::new(context);
    let beatmaps_context = context_arc.clone();
    let beatmapsets_context = context_arc.clone();

    let res = tokio::try_join!(
        tokio::spawn(async move { crawl_beatmaps(beatmaps_context.as_ref()).await }),
        tokio::spawn(async move { crawl_beatmapsets(beatmapsets_context.as_ref()).await })
    );
    if !res.is_ok() {
        anyhow::bail!("crawler failed: {:?}", res);
    }

    Ok(())
}
