use serde::Deserialize;

use crate::converter::livesplit::livesplitsegments::Segments;

#[derive(Debug, Deserialize)]
#[serde(rename = "Run")]
pub struct LiveSplitRun {
    #[serde(rename = "GameName", default)]
    pub game_name: String,

    #[serde(rename = "CategoryName", default)]
    pub category_name: String,

    #[serde(rename = "Platform", default)]
    pub platform: String,

    #[serde(rename = "AttemptCount", default)]
    pub attempt_count: i32,

    #[serde(rename = "Segments", default)]
    pub segments: Segments,
}
