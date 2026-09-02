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
    pub fn from_titles_list(title: String, titles: Vec<String>) -> Self {
        let mut splits: Vec<LibreSplitSplit> = vec![];
        for title in titles.iter() {
            let split = LibreSplitSplit::from_title(title.to_string());
            splits.push(split);
        }

        Self {
            title,
            attempt_count: 0,
            comparison_method: 0,
            start_delay: "0.000000".to_string(),
            world_record: "0.000000".to_string(),
            splits,
            theme: "standard".to_string(),
            theme_variant: "standard".to_string(),
            width: 10,
            height: 10,
        }
    }

    pub fn to_json(&self) -> String {
        to_string_pretty(&self).expect("Failed to write JSON.")
    }
}
