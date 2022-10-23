use rosu_v2::prelude::{Beatmapset, OsuError};
use std::collections::HashMap;

use crate::Context;

pub async fn fetch(
    ctx: &Context,
    beatmapset_id: u32,
) -> anyhow::Result<Option<Beatmapset>, OsuError> {
    let beatmapset_result = ctx.osu_api.beatmapset(beatmapset_id).await;

    match beatmapset_result {
        Ok(beatmapset) => Ok(Some(beatmapset)),
        Err(OsuError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

// TODO: authenticating twice is stupid
// TODO: store?
pub async fn download(
    ctx: &Context,
    beatmapset_id: u32,
) -> anyhow::Result<hyper::Response<hyper::Body>> {
    let client = reqwest::Client::new();

    let mut body_params = HashMap::new();
    body_params.insert("username", ctx.config.osu_username.clone());
    body_params.insert("password", ctx.config.osu_password.clone());
    body_params.insert("client_id", "5".to_string()); // lazer client id
    body_params.insert(
        "client_secret",
        "FGc9GAtyHzeQDshWP5Ah7dega8hJACAJpQtw6OXk".to_string(), // lazer client secret
    );
    body_params.insert("grant_type", "password".to_string());
    body_params.insert("scope", "*".to_string());

    let api_response: serde_json::Value = client
        .post("https://osu.ppy.sh/oauth/token")
        .json(&body_params)
        .header("User-Agent", "osu!")
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    let access_token = api_response["access_token"].as_str().unwrap().to_string();

    let redirect_request = hyper::Request::builder()
        .uri(format!(
            "https://osu.ppy.sh/api/v2/beatmapsets/{}/download",
            beatmapset_id
        ))
        .header("Content-Type", "application/x-osu-beatmap-archive")
        .header("Accept", "application/x-osu-beatmap-archive")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "osu!")
        .body(hyper::Body::empty())?;

    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    let redirect_response = client.request(redirect_request).await?;
    let mut osz_response = redirect_response;
    let redirect_url = osz_response.headers().get("Location");
    if let Some(redirect_url) = redirect_url {
        osz_response = client.get(redirect_url.to_str()?.parse()?).await?;
    }

    Ok(osz_response)
}
