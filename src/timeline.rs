use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Timeline {
    pub fade_in: Duration,
    pub hold: Duration,
    pub fade_out: Duration,
}
