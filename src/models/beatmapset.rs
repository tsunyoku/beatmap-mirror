use chrono::{DateTime, Utc};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Beatmapset {
    pub data: rosu_v2::prelude::Beatmapset,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_checked: DateTime<Utc>,
    pub crawled: bool,
}
