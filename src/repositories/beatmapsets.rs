use crate::{
    helpers::elastic::{self, ElasticDocument},
    models::{beatmapset::Beatmapset, mode::Mode, ranked_status::RankedStatus},
    Context,
};

use elasticsearch::SearchParts;

pub async fn create(ctx: &Context, beatmapset: Beatmapset) -> anyhow::Result<()> {
    let elastic_document = ElasticDocument {
        id: beatmapset.data.mapset_id.to_string(),
        data: beatmapset,
    };

    elastic::create(
        &ctx.database,
        &ctx.config.elastic_beatmapsets_index,
        elastic_document,
    )
    .await?;

    Ok(())
}

pub async fn search(
    ctx: &Context,
    query: Option<String>,
    amount: u64,
    offset: u64,
    status: RankedStatus,
    mode: Mode,
) -> anyhow::Result<Vec<Beatmapset>> {
    let mut query_conditions: Vec<serde_json::Value> = Vec::new();

    if let Some(query) = query {
        let field_query = serde_json::json!({
            "simple_query_string": {
                "query": query,
                "fields": vec![
                    "data.artist",
                    "data.creator",
                    "data.title",
                    "data.title_unicode",
                    "data.tags",
                    "data.beatmaps.version"
                ]
            }
        });

        query_conditions.push(field_query);
    }

    if mode != Mode::All {
        query_conditions.push(serde_json::json!({
            "term": {
                "data.beatmaps.mode": mode as i8
            }
        }));
    }

    if status != RankedStatus::All {
        query_conditions.push(serde_json::json!({
            "term": {
                "data.status": status as i8
            }
        }));
    }

    let json_query =
        serde_json::json!({"query": {"bool": {"must": serde_json::json!(query_conditions)}}});

    let elastic_response = ctx
        .database
        .search(SearchParts::Index(&[&ctx.config.elastic_beatmapsets_index]))
        .body(json_query)
        .size(amount as i64)
        .from(offset as i64)
        .send()
        .await?;

    let beatmapsets_json = elastic_response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| anyhow::anyhow!("failed to search for beatmap: {}", e))?;

    let beatmaps: Vec<Beatmapset> = beatmapsets_json
        .pointer("/hits/hits")
        .unwrap_or(&serde_json::Value::Array(vec![]))
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| {
            v.pointer("/_source")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
        })
        .collect();

    Ok(beatmaps)
}

pub async fn update(ctx: &Context, beatmapset: Beatmapset) -> anyhow::Result<()> {
    let elastic_document = ElasticDocument {
        id: beatmapset.data.mapset_id.to_string(),
        data: beatmapset,
    };

    elastic::update(
        &ctx.database,
        &ctx.config.elastic_beatmapsets_index,
        elastic_document,
    )
    .await?;

    Ok(())
}
