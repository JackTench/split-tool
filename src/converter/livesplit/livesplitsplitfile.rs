use crate::converter::livesplit::livesplitsegments::Segments;

pub struct LiveSplitRun {
    pub game_name: String,
    pub category_name: String,
    pub platform: String,
    pub attempt_count: i32,
    pub segments: Segments,
}
