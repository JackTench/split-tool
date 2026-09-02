use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibreSplitSplit {
    pub title: String,
    pub icon: String,
    pub time: String,
    pub best_time: String,
    pub best_segment: String,
}

impl LibreSplitSplit {
    pub fn from_title(title: String) -> Self {
        Self {
            title,
            icon: String::new(),
            time: "0.000000".to_string(),
            best_time: "0.000000".to_string(),
            best_segment: "0.000000".to_string(),
        }
    }
}
