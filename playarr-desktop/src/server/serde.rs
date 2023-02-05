use serde::Deserialize;
use serde::Serialize;

// generated using https://transform.tools/json-to-rust-serde

pub type Shows = Vec<Show>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Show {
    pub title: String,
    pub alternate_titles: Vec<AlternateTitle>,
    pub sort_title: String,
    pub status: String,
    pub ended: bool,
    pub overview: String,
    pub next_airing: Option<String>,
    pub previous_airing: Option<String>,
    pub network: String,
    pub air_time: Option<String>,
    pub images: Vec<Image>,
    pub seasons: Vec<Season>,
    pub year: i64,
    pub path: String,
    pub quality_profile_id: i64,
    pub language_profile_id: i64,
    pub season_folder: bool,
    pub monitored: bool,
    pub use_scene_numbering: bool,
    pub runtime: i64,
    pub tvdb_id: i64,
    pub tv_rage_id: i64,
    pub tv_maze_id: i64,
    pub first_aired: String,
    pub series_type: String,
    pub clean_title: String,
    pub imdb_id: String,
    pub title_slug: String,
    pub root_folder_path: String,
    pub certification: Option<String>,
    pub genres: Vec<String>,
    pub tags: Vec<i64>,
    pub added: String,
    pub ratings: Ratings,
    pub statistics: Statistics2,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateTitle {
    pub title: String,
    pub scene_season_number: Option<i64>,
    pub season_number: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub cover_type: String,
    pub url: String,
    pub remote_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub season_number: i64,
    pub monitored: bool,
    pub statistics: Statistics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub episode_file_count: i64,
    pub episode_count: i64,
    pub total_episode_count: i64,
    pub size_on_disk: i64,
    pub release_groups: Vec<String>,
    pub percent_of_episodes: f64,
    pub next_airing: Option<String>,
    pub previous_airing: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    pub votes: i64,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics2 {
    pub season_count: i64,
    pub episode_file_count: i64,
    pub episode_count: i64,
    pub total_episode_count: i64,
    pub size_on_disk: i64,
    pub release_groups: Vec<String>,
    pub percent_of_episodes: f64,
}

// episodes endpoint serdeified

pub type Episodes = Vec<Episode>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub series_id: i64,
    pub tvdb_id: i64,
    pub episode_file_id: i64,
    pub season_number: i64,
    pub episode_number: i64,
    pub title: String,
    pub air_date: Option<String>,
    pub air_date_utc: Option<String>,
    pub overview: Option<String>,
    pub has_file: bool,
    pub monitored: bool,
    pub absolute_episode_number: Option<i64>,
    pub scene_absolute_episode_number: Option<i64>,
    pub scene_episode_number: Option<i64>,
    pub scene_season_number: Option<i64>,
    pub unverified_scene_numbering: bool,
    pub images: Vec<EpisodeImage>,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeImage {
    pub cover_type: String,
    pub url: String,
}
