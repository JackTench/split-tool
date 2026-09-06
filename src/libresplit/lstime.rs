use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibreSplitTime {
    pub real_time: String,
    pub game_time: String,
}

impl Default for LibreSplitTime {
    fn default() -> Self {
        Self {
            real_time: "0.000000".to_string(),
            game_time: "0.000000".to_string(),
        }
    }
}
