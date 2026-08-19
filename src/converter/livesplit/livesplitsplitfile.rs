use crate::converter::livesplit::livesplitsplit::LiveSplitSplit;

pub struct LiveSplitSplitFile {
    pub game_name: String,
    pub category_name: String,
    pub platform: String,
    pub attempt_count: u32,
    pub segments: Vec<LiveSplitSplit>,
}
