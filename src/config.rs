#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub api_port: u16,

    #[clap(long, env)]
    pub app_component: String,

    #[clap(long, env)]
    pub elastic_host: String,

    #[clap(long, env)]
    pub elastic_port: u16,

    #[clap(long, env)]
    pub elastic_user: String,

    #[clap(long, env)]
    pub elastic_password: String,

    #[clap(long, env)]
    pub elastic_beatmaps_index: String,

    #[clap(long, env)]
    pub elastic_beatmapsets_index: String,

    #[clap(long, env)]
    pub osu_api_client_id: u64,

    #[clap(long, env)]
    pub osu_api_client_secret: String,

    #[clap(long, env)]
    pub osu_username: String,

    #[clap(long, env)]
    pub osu_password: String,

    #[clap(long, env)]
    pub backoff_start: f64,

    #[clap(long, env)]
    pub max_backoff: f64,

    #[clap(long, env)]
    pub max_requests_per_second: u32,

    #[clap(long, env)]
    pub max_retries: usize,

    #[clap(long, env)]
    pub max_timeout: u64,
}
