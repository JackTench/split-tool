use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Segments {
    #[serde(rename = "Segment", default)]
    pub segment: Vec<Segment>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Segment {
    #[serde(rename = "Name", default)]
    pub name: String,

    #[serde(rename = "SplitTimes", default)]
    pub split_times: SplitTimes,

    #[serde(rename = "BestSegmentTime", default)]
    pub best_segment_time: Time,
}

#[derive(Debug, Deserialize, Default)]
pub struct SplitTimes {
    #[serde(rename = "SplitTime", default)]
    pub split_time: Vec<SplitTime>,
}

#[derive(Debug, Deserialize, Default)]
pub struct SplitTime {
    #[serde(rename = "RealTime", default)]
    pub real_time: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Time {
    #[serde(rename = "RealTime", default)]
    pub real_time: String,
}
