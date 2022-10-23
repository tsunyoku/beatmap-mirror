use elasticsearch::{
    indices::{IndicesCreateParts, IndicesExistsParts},
    BulkOperation, BulkOperations, CreateParts, Elasticsearch, UpdateParts,
};
use serde::Serialize;

pub async fn index_exists(database: &Elasticsearch, index: &str) -> anyhow::Result<bool> {
    let result = database
        .indices()
        .exists(IndicesExistsParts::Index(&vec![index]))
        .send()
        .await?
        .status_code()
        == 200;

    Ok(result)
}

pub async fn create_index(database: &Elasticsearch, index: &str) -> anyhow::Result<()> {
    database
        .indices()
        .create(IndicesCreateParts::Index(index))
        .send()
        .await?;

    Ok(())
}

pub async fn create_index_if_not_exists(
    database: &Elasticsearch,
    index: &str,
) -> anyhow::Result<()> {
    let should_create = !index_exists(database, index).await?;
    if should_create {
        create_index(database, index).await?;
    }

    Ok(())
}

pub struct ElasticDocument<T: Serialize> {
    pub id: String,
    pub data: T,
}

pub async fn create<T: Serialize>(
    database: &Elasticsearch,
    index: &str,
    document: ElasticDocument<T>,
) -> anyhow::Result<()> {
    database
        .create(CreateParts::IndexId(index, &document.id))
        .body(document.data)
        .send()
        .await?;

    Ok(())
}

pub async fn bulk_create<T: Serialize>(
    database: &Elasticsearch,
    index: &str,
    data: Vec<ElasticDocument<T>>,
) -> anyhow::Result<()> {
    let mut operations = BulkOperations::new();
    for document in data {
        let create_operation =
            BulkOperation::create(document.id.as_str(), document.data).index(index);

        operations.push(create_operation)?;
    }

    database
        .bulk(elasticsearch::BulkParts::None)
        .body(vec![operations])
        .send()
        .await?;

    Ok(())
}

pub async fn update<T: Serialize>(
    database: &Elasticsearch,
    index: &str,
    document: ElasticDocument<T>,
) -> anyhow::Result<()> {
    database
        .update(UpdateParts::IndexId(index, &document.id))
        .body(document.data)
        .send()
        .await?;

    Ok(())
}
