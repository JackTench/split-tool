use crate::{
    converter::{ConvertableSplitFile, livesplit::livesplitsplitfile::LiveSplitRun},
    libresplit::{lssplit::LibreSplitSplit, lssplitfile::LibreSplitSplitFile},
};

pub mod livesplitsegments;
pub mod livesplitsplitfile;

impl ConvertableSplitFile for LiveSplitRun {
    fn convert(&self) -> LibreSplitSplitFile {
        let splits = self
            .segments
            .segment
            .into_iter()
            .map(|segment| {
                let split_time = segment
                    .split_times
                    .split_time
                    .first()
                    .map(|time| time.real_time.clone())
                    .unwrap_or_else(|| "0.000000".to_string());

                let best_segment = if segment.best_segment_time.real_time.is_empty() {
                    "0.000000".to_string()
                } else {
                    segment.best_segment_time.real_time
                };

                LibreSplitSplit {
                    title: segment.name,
                    // TODO: Port icons.
                    icon: String::new(),
                    time: split_time.clone(),
                    best_time: split_time,
                    best_segment,
                }
            })
            .collect();

        LibreSplitSplitFile {
            title: format!("{} {}", self.game_name, self.category_name),
            attempt_count: self.attempt_count,
            comparison_method: 0,
            start_delay: "0.000000".to_string(),
            world_record: "0.000000".to_string(),
            splits,
            theme: "default".to_string(),
            theme_variant: "default".to_string(),
            width: 10,
            height: 10,
        }
    }
}
