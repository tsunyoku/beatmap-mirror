use crate::{
    helpers::elastic::{self, ElasticDocument},
    models::beatmap::Beatmap,
    Context,
};

pub async fn create(ctx: &Context, beatmap: Beatmap) -> anyhow::Result<()> {
    let elastic_document = ElasticDocument {
        id: beatmap.data.map_id.to_string(),
        data: beatmap,
    };

    elastic::create(
        &ctx.database,
        &ctx.config.elastic_beatmaps_index,
        elastic_document,
    )
    .await?;

    Ok(())
}

pub async fn bulk_create(ctx: &Context, beatmaps: Vec<Beatmap>) -> anyhow::Result<()> {
    let mut elastic_beatmaps: Vec<ElasticDocument<Beatmap>> = Vec::with_capacity(beatmaps.len());
    for beatmap in beatmaps {
        let elastic_document = ElasticDocument {
            id: beatmap.data.map_id.to_string(),
            data: beatmap,
        };

        elastic_beatmaps.push(elastic_document);
    }

    elastic::bulk_create(
        &ctx.database,
        &ctx.config.elastic_beatmaps_index,
        elastic_beatmaps,
    )
    .await?;

    Ok(())
}

pub async fn update(ctx: &Context, beatmap: Beatmap) -> anyhow::Result<()> {
    let elastic_document = ElasticDocument {
        id: beatmap.data.map_id.to_string(),
        data: beatmap,
    };

    elastic::update(
        &ctx.database,
        &ctx.config.elastic_beatmaps_index,
        elastic_document,
    )
    .await?;

    Ok(())
}
