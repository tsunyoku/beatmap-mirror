use std::sync::Arc;

use config::Config;
use elasticsearch::Elasticsearch;
use rosu_v2::Osu;

pub mod api;
pub mod config;
pub mod crawler;
pub mod helpers;
pub mod models;
pub mod repositories;
pub mod updater;
pub mod usecases;

#[derive(Clone)]
pub struct Context {
    pub config: Arc<Config>,
    pub database: Elasticsearch,
    pub osu_api: Arc<Osu>,
}
