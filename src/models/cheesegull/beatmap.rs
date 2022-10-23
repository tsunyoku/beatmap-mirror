use rosu_v2::prelude::{Beatmap, Beatmapset};

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CheesegullBeatmap {
    #[serde(rename = "FileMD5")]
    pub file_md5: String,

    #[serde(rename = "TotalLength")]
    pub total_length: u32,

    #[serde(rename = "Playcount")]
    pub playcount: u32,

    #[serde(rename = "Mode")]
    pub mode: u8,

    #[serde(rename = "HP")]
    pub hp: f32,

    #[serde(rename = "MaxCombo")]
    pub max_combo: u32,

    #[serde(rename = "ParentSetID")]
    pub parent_set_id: u32,

    #[serde(rename = "CS")]
    pub cs: f32,

    #[serde(rename = "AR")]
    pub ar: f32,

    #[serde(rename = "OD")]
    pub od: f32,

    #[serde(rename = "BeatmapID")]
    pub beatmap_id: u32,

    #[serde(rename = "HitLength")]
    pub hit_length: u32,

    #[serde(rename = "DifficultyRating")]
    pub difficulty_rating: f32,

    #[serde(rename = "Passcount")]
    pub passcount: u32,

    #[serde(rename = "DiffName")]
    pub diff_name: String,

    #[serde(rename = "BPM")]
    pub bpm: f32,
}

impl CheesegullBeatmap {
    pub fn from_beatmapset(osu_beatmap: &Beatmap, osu_beatmapset: &Beatmapset) -> Self {
        Self {
            file_md5: osu_beatmap.checksum.clone().unwrap_or(String::from("")),
            total_length: osu_beatmap.seconds_total,
            playcount: osu_beatmapset.playcount,
            mode: osu_beatmap.mode as u8,
            hp: osu_beatmap.hp,
            max_combo: osu_beatmap.max_combo.unwrap_or(0),
            parent_set_id: osu_beatmap.mapset_id,
            cs: osu_beatmap.cs,
            ar: osu_beatmap.ar,
            od: osu_beatmap.od,
            beatmap_id: osu_beatmap.map_id,
            hit_length: osu_beatmap.seconds_drain,
            difficulty_rating: osu_beatmap.stars,
            passcount: osu_beatmap.passcount,
            diff_name: osu_beatmap.version.clone(),
            bpm: osu_beatmap.bpm,
        }
    }
}

impl From<Beatmap> for CheesegullBeatmap {
    fn from(osu_beatmap: Beatmap) -> Self {
        let beatmapset = osu_beatmap.mapset.unwrap();

        Self {
            file_md5: osu_beatmap.checksum.unwrap_or(String::from("")),
            total_length: osu_beatmap.seconds_total,
            playcount: beatmapset.playcount,
            mode: osu_beatmap.mode as u8,
            hp: osu_beatmap.hp,
            max_combo: osu_beatmap.max_combo.unwrap_or(0),
            parent_set_id: osu_beatmap.mapset_id,
            cs: osu_beatmap.cs,
            ar: osu_beatmap.ar,
            od: osu_beatmap.od,
            beatmap_id: osu_beatmap.map_id,
            hit_length: osu_beatmap.seconds_drain,
            difficulty_rating: osu_beatmap.stars,
            passcount: osu_beatmap.passcount,
            diff_name: osu_beatmap.version,
            bpm: osu_beatmap.bpm,
        }
    }
}
