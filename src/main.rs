use std::sync::Arc;

use beatmap_mirror::{api, config::Config, crawler, updater, Context};
use clap::Parser;
use elasticsearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    Elasticsearch,
};
use rosu_v2::Osu;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Config::parse();

    let credentials =
        Credentials::Basic(config.elastic_user.clone(), config.elastic_password.clone());

    let url = Url::parse(&format!(
        "http://{}:{}",
        config.elastic_host, config.elastic_port
    ))?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool)
        .auth(credentials)
        .cert_validation(CertificateValidation::None)
        .build()?;

    let database = Elasticsearch::new(transport);

    let osu_api = Osu::new(
        config.osu_api_client_id,
        config.osu_api_client_secret.clone(),
    )
    .await?;

    let ctx = Context {
        config: Arc::new(config),
        database,
        osu_api: Arc::new(osu_api),
    };

    match ctx.config.app_component.as_str() {
        "crawler" => crawler::serve(ctx).await?,
        "api" => api::serve(ctx).await?,
        "updater" => updater::serve(ctx).await?,
        _ => anyhow::bail!("unknown app component: {}", ctx.config.app_component),
    }

    Ok(())
}
