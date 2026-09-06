use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibreSplitTime {
    pub real_time: String,
    pub game_time: String,
}
