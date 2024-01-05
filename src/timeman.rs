use crate::search::*;
use crate::types::*;
use crate::usioption::*;

#[derive(Clone)]
pub struct TimeManagement {
    start_time: Option<std::time::Instant>,
    optimum_time_milli: std::time::Duration,
    maximum_time_milli: std::time::Duration,
}

impl TimeManagement {
    pub fn new() -> TimeManagement {
        TimeManagement {
            start_time: None,
            optimum_time_milli: std::time::Duration::from_millis(0),
            maximum_time_milli: std::time::Duration::from_millis(0),
        }
    }
    pub fn init(&mut self, usi_optoins: &UsiOptions, limits: &mut LimitsType, us: Color, ply: i32) {
        self.start_time = limits.start_time;
        let moves_to_go = 50;
        let move_overhead = 10;
        let slow_mover = usi_optoins.get_i64(UsiOptions::SLOW_MOVER);
        let time_left = std::cmp::max(
            1,
            limits.time[us.0 as usize].as_millis() as i64 + limits.inc[us.0 as usize].as_millis() as i64 * (moves_to_go - 1)
                - move_overhead * (2 + moves_to_go),
        );
        let time_left = time_left * slow_mover / 100;

        let opt_scale = ((0.8 + ply as f64 / 128.0) / moves_to_go as f64)
            .min(0.8 * limits.time[us.0 as usize].as_millis() as f64 / time_left as f64);
        let max_scale = 6.3f64.min(1.5 + 0.11 * moves_to_go as f64);

        fn min(x: f64, y: f64) -> f64 {
            x.min(y)
        }
        self.optimum_time_milli = std::time::Duration::from_millis((opt_scale * time_left as f64) as u64);
        self.maximum_time_milli = std::time::Duration::from_millis(min(
            0.8 * limits.time[us.0 as usize].as_millis() as f64 - move_overhead as f64,
            max_scale * self.optimum_time_milli.as_millis() as f64,
        ) as u64);
    }
    pub fn optimum_millis(&self) -> i64 {
        self.optimum_time_milli.as_millis() as i64
    }
    pub fn maximum_millis(&self) -> i64 {
        self.maximum_time_milli.as_millis() as i64
    }
    pub fn elapsed(&self) -> i64 {
        let duration = self.start_time.unwrap().elapsed();
        (duration.as_secs() * 1000 + u64::from(duration.subsec_millis())) as i64
    }
}
