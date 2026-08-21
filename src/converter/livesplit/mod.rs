use crate::{
    converter::{ConvertableSplitFile, livesplit::livesplitsplitfile::LiveSplitSplitFile},
    libresplit::{lssplit::LibreSplitSplit, lssplitfile::LibreSplitSplitFile},
};

pub mod livesplitsplit;
pub mod livesplitsplitfile;

impl ConvertableSplitFile for LiveSplitSplitFile {
    fn convert(&self) -> LibreSplitSplitFile {
        // Get title.
        let title = self.game_name.clone() + " " + &self.category_name;
        let attempt_count = self.attempt_count;

        // Construct splits vector.
        let mut splits: Vec<LibreSplitSplit> = vec![];
        for lss_split in self.segments.clone() {
            let split = LibreSplitSplit {
                title: lss_split.name,
                icon: "".to_string(),
                time: lss_split.split_time,
                best_time: "".to_string(),
                best_segment: lss_split.best_segment,
            };
            splits.push(split);
        }

        // Get size.
        // The window of LibreSplit will not shrink beyond this size.
        let width = 60;
        let height = 80;

        LibreSplitSplitFile {
            title,
            attempt_count,
            comparison_method: 0,
            start_delay: "".to_string(),
            world_record: "".to_string(),
            splits,
            theme: "".to_string(),
            theme_variant: "".to_string(),
            width,
            height,
        }
    }
}
