use chrono::{DateTime, Utc};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Beatmap {
    pub data: rosu_v2::prelude::Beatmap,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_checked: DateTime<Utc>,
}
