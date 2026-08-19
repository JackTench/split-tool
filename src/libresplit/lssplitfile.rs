use serde::{Deserialize, Serialize};

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
