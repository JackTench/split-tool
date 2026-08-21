pub struct Segments {
    pub segment: Vec<Segment>,
}

pub struct Segment {
    pub name: String,
    pub split_times: SplitTimes,
    pub best_segment_time: Time,
}

pub struct SplitTimes {
    pub split_time: Vec<SplitTime>,
}

pub struct SplitTime {
    pub real_time: String,
}

pub struct Time {
    real_time: String,
}
