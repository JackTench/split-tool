use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use crate::libresplit::lssplit::LibreSplitSplit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibreSplitSplitFile {
    pub title: String,
    pub attempt_count: i32,
    pub comparison_method: i32,
    pub start_delay: String,
    pub world_record: String,
    pub splits: Vec<LibreSplitSplit>,
    pub theme: String,
    pub theme_variant: String,
    pub width: i32,
    pub height: i32,
}

impl LibreSplitSplitFile {
    pub fn to_json(&self) -> String {
        to_string_pretty(&self).expect("Failed to write JSON.")
    }
}
