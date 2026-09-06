use serde::{Deserialize, Serialize};

use crate::libresplit::lstime::LibreSplitTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibreSplitSplit {
    pub title: String,
    pub icon: String,
    pub time: LibreSplitTime,
    pub best_time: LibreSplitTime,
    pub best_segment: LibreSplitTime,
}

impl LibreSplitSplit {
    pub fn from_title(title: String) -> Self {
        Self {
            title,
            icon: String::new(),
            time: LibreSplitTime::default(),
            best_time: LibreSplitTime::default(),
            best_segment: LibreSplitTime::default(),
        }
    }
}
