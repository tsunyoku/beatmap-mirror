use chrono::{TimeZone, Utc};
use rosu_v2::prelude::Beatmapset;

use super::beatmap::CheesegullBeatmap;

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CheesegullBeatmapset {
    #[serde(rename = "SetID")]
    pub set_id: u32,

    #[serde(rename = "RankedStatus")]
    pub ranked_status: u8,

    #[serde(rename = "ChildrenBeatmaps")]
    pub children_beatmaps: Vec<CheesegullBeatmap>,

    #[serde(rename = "ApprovedDate")]
    pub approved_date: Option<String>,

    #[serde(rename = "LastUpdate")]
    pub last_update: String,

    #[serde(rename = "Artist")]
    pub artist: String,

    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "Creator")]
    pub creator: String,

    #[serde(rename = "CreatorID")]
    pub creator_id: u32,

    #[serde(rename = "Source")]
    pub source: String,

    #[serde(rename = "Tags")]
    pub tags: String,

    #[serde(rename = "HasVideo")]
    pub has_video: bool,

    #[serde(rename = "Genre")]
    pub genre: u8,

    #[serde(rename = "Language")]
    pub language: u8,

    #[serde(rename = "Favourites")]
    pub favourites: u32,

    #[serde(rename = "StarRating")]
    pub star_rating: f32,
}

impl From<Beatmapset> for CheesegullBeatmapset {
    fn from(beatmapset: Beatmapset) -> Self {
        let beatmaps = beatmapset.maps.as_ref().unwrap();

        let highest_sr = beatmaps
            .iter()
            .map(|map| map.stars)
            .fold(f32::NEG_INFINITY, f32::max);

        Self {
            set_id: beatmapset.mapset_id,
            ranked_status: beatmapset.status as u8,
            children_beatmaps: beatmaps
                .into_iter()
                .map(|beatmap| CheesegullBeatmap::from_beatmapset(beatmap, &beatmapset))
                .collect(),
            approved_date: beatmapset
                .ranked_date
                .map(|date| Utc.timestamp(date.unix_timestamp(), 0).to_rfc3339()),
            last_update: Utc
                .timestamp(beatmapset.last_updated.unix_timestamp(), 0)
                .to_rfc3339(),
            artist: beatmapset.artist,
            title: beatmapset.title,
            creator: beatmapset.creator_name.into_string(),
            creator_id: beatmapset.creator_id,
            source: beatmapset.source,
            tags: beatmapset.tags,
            has_video: beatmapset.video,
            genre: beatmapset.genre.map(|g| g as u8).unwrap_or(0),
            language: beatmapset.language.map(|l| l as u8).unwrap_or(0),
            favourites: beatmapset.favourite_count,
            star_rating: highest_sr,
        }
    }
}
