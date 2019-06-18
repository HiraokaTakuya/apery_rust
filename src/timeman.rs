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
    const MOVE_HORIZON: u64 = 50;
    const MAX_RATIO: f64 = 7.3;
    const STEAL_RATIO: f64 = 0.34;

    pub fn new() -> TimeManagement {
        TimeManagement {
            start_time: None,
            optimum_time_milli: std::time::Duration::from_millis(0),
            maximum_time_milli: std::time::Duration::from_millis(0),
        }
    }
    fn move_importance(ply: i32) -> f64 {
        const X_SCALE: f64 = 6.85;
        const X_SHIFT: f64 = 65.5;
        const SKEW: f64 = 0.171;
        (1.0 + (f64::from(ply) - X_SHIFT) / X_SCALE)
            .exp()
            .powf(-SKEW)
            + std::f64::MIN_POSITIVE
    }
    fn remaining_base(
        max_ratio: f64,
        steal_ratio: f64,
        my_time: std::time::Duration,
        moves_to_go: u64,
        ply: i32,
        slow_mover: i64,
    ) -> std::time::Duration {
        let move_importance = (Self::move_importance(ply) * slow_mover as f64) / 100.0;
        let other_moves_importance =
            (1..moves_to_go).fold(0.0, |sum, i| sum + Self::move_importance(ply + i as i32));
        let ratio1 =
            (max_ratio * move_importance) / (max_ratio * move_importance + other_moves_importance);
        let ratio2 = (move_importance + steal_ratio * other_moves_importance)
            / (move_importance + other_moves_importance);
        std::time::Duration::from_millis((my_time.as_millis() as f64 * ratio1.min(ratio2)) as u64)
    }
    fn remaining_optimum(
        my_time: std::time::Duration,
        moves_to_go: u64,
        ply: i32,
        slow_mover: i64,
    ) -> std::time::Duration {
        TimeManagement::remaining_base(1.0, 0.0, my_time, moves_to_go, ply, slow_mover)
    }
    fn remaining_maximum(
        my_time: std::time::Duration,
        moves_to_go: u64,
        ply: i32,
        slow_mover: i64,
    ) -> std::time::Duration {
        TimeManagement::remaining_base(
            TimeManagement::MAX_RATIO,
            TimeManagement::STEAL_RATIO,
            my_time,
            moves_to_go,
            ply,
            slow_mover,
        )
    }
    pub fn init(&mut self, usi_optoins: &UsiOptions, limits: &LimitsType, us: Color, ply: i32) {
        self.start_time = limits.start_time;
        let min_thinking_time = usi_optoins.get_i64("Minimum_Thinking_Time") as u64;
        self.optimum_time_milli = std::cmp::max(
            limits.time[us.0 as usize],
            std::time::Duration::from_millis(min_thinking_time),
        );
        self.maximum_time_milli = self.optimum_time_milli;
        let slow_mover = usi_optoins.get_i64("Slow_Mover");
        let max_moves_to_go = TimeManagement::MOVE_HORIZON;
        let move_overhead = 0;
        for hypothetical_moves_to_go in 1..max_moves_to_go {
            let hypothetical_my_time = limits.time[us.0 as usize]
                + limits.inc[us.0 as usize] * (hypothetical_moves_to_go - 1) as u32
                - move_overhead
                    * std::time::Duration::from_millis(
                        2 + std::cmp::min(hypothetical_moves_to_go, 40),
                    );
            let hypothetical_my_time =
                std::cmp::max(hypothetical_my_time, std::time::Duration::from_millis(0));
            let t1 = std::time::Duration::from_millis(min_thinking_time)
                + TimeManagement::remaining_optimum(
                    hypothetical_my_time,
                    hypothetical_moves_to_go,
                    ply,
                    slow_mover,
                );
            let t2 = std::time::Duration::from_millis(min_thinking_time)
                + TimeManagement::remaining_maximum(
                    hypothetical_my_time,
                    hypothetical_moves_to_go,
                    ply,
                    slow_mover,
                );

            self.optimum_time_milli = std::cmp::min(t1, self.optimum_time_milli);
            self.maximum_time_milli = std::cmp::min(t2, self.maximum_time_milli);
        }
        // not use ponder bonus
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
