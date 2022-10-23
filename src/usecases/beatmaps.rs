use crate::Context;
use crate::{models::beatmap::Beatmap, repositories};
use elasticsearch::SearchParts;
use elasticsearch_dsl::{Query, Search};

pub async fn fetch(ctx: &Context, beatmap_id: u32) -> anyhow::Result<Option<Beatmap>> {
    let query = Search::new()
        .query(Query::term("data.id", beatmap_id))
        .size(1);

    let elastic_response = ctx
        .database
        .search(SearchParts::Index(&[&ctx.config.elastic_beatmaps_index]))
        .body(query)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("failed to search for beatmap: {}", e))?;

    let beatmap: Option<Beatmap> = elastic_response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| anyhow::anyhow!("failed to search for beatmap: {}", e))?
        .pointer("/hits/hits/0/_source")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    if beatmap.is_some() {
        return Ok(beatmap);
    }

    let osu_beatmap = repositories::osu::beatmaps::fetch(ctx, beatmap_id)
        .await
        .map_err(|e| anyhow::anyhow!("failed to search for beatmap: {}", e))?;

    Ok(match osu_beatmap {
        Some(osu_beatmap) => {
            let now = chrono::Utc::now();
            let beatmap = Beatmap {
                data: osu_beatmap,
                created_at: now,
                updated_at: now,
                last_checked: now,
            };

            repositories::beatmaps::create(ctx, beatmap.clone()).await?;
            Some(beatmap)
        }
        _ => None,
    })
}
