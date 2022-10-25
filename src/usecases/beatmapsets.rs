use crate::{models::beatmapset::Beatmapset, repositories, Context};
use chrono::{TimeZone, Utc};
use elasticsearch::SearchParts;
use elasticsearch_dsl::{Query, Search};
use rosu_v2::prelude::Beatmapset as OsuBeatmapset;

pub async fn fetch(ctx: &Context, beatmapset_id: u32) -> anyhow::Result<Option<Beatmapset>> {
    let query = Search::new()
        .query(Query::term("data.id", beatmapset_id))
        .size(1);

    let elastic_response = ctx
        .database
        .search(SearchParts::Index(&[&ctx.config.elastic_beatmapsets_index]))
        .body(query)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("failed to search for beatmapset: {}", e))?;

    let beatmapset: Option<Beatmapset> = elastic_response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| anyhow::anyhow!("failed to search for beatmapset: {}", e))?
        .pointer("/hits/hits/0/_source")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    if beatmapset.is_some() {
        return Ok(beatmapset);
    }

    let osu_beatmapset = repositories::osu::beatmapsets::fetch(ctx, beatmapset_id)
        .await
        .map_err(|e| anyhow::anyhow!("failed to search for beatmapset: {}", e))?;

    Ok(match osu_beatmapset {
        Some(osu_beatmapset) => {
            let now = chrono::Utc::now();
            let beatmapset = Beatmapset {
                data: osu_beatmapset,
                created_at: now,
                updated_at: now,
                last_checked: now,
                crawled: false,
            };

            repositories::beatmapsets::create(ctx, beatmapset.clone()).await?;
            Some(beatmapset)
        }
        _ => None,
    })
}

pub fn format_to_direct(beatmapsets: Vec<OsuBeatmapset>) -> String {
    let mut response_lines: Vec<String> = Vec::with_capacity(beatmapsets.len());

    for beatmapset in beatmapsets {
        let difficulties_string = beatmapset
            .maps
            .unwrap_or(vec![])
            .iter()
            .map(|beatmap| {
                format!(
                    "[{}⭐] {} // cs: {} / od: {} / ar: {} / hp: {}@{}",
                    beatmap.stars,
                    beatmap.version,
                    beatmap.cs,
                    beatmap.od,
                    beatmap.ar,
                    beatmap.hp,
                    beatmap.mode as u8
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        let last_updated = Utc.timestamp(beatmapset.last_updated.unix_timestamp(), 0);
        let last_updated_str = last_updated.to_rfc3339();
        let beatmapset_string = format!(
            "{}.osz|{}|{}|{}|{}|10.0|{}|{}|0|{}|0|0|0|{}",
            beatmapset.mapset_id,
            beatmapset.artist,
            beatmapset.title,
            beatmapset.creator_name,
            beatmapset.status as u8,
            last_updated_str,
            beatmapset.mapset_id,
            beatmapset.video as u8,
            difficulties_string
        );

        response_lines.push(beatmapset_string);
    }

    format!("{}\n{}", response_lines.len(), response_lines.join("\n"))
}
