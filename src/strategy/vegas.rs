/*
 * Created by: Cleiton Augusto Correa Bezerra
 * Project: FlowGuard - Adaptive Backpressure for Rust
 * Algorithm: Optimized TCP Vegas for Concurrency Control
 */

use crate::LimitStrategy;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

pub struct VegasStrategy {
    current_limit: AtomicUsize,
    base_rtt: RwLock<Duration>,
    alpha: f64,
    beta: f64,
    min_limit: usize,
    max_limit: usize,
}

impl VegasStrategy {
    pub fn new(initial_limit: usize) -> Self {
        Self {
            current_limit: AtomicUsize::new(initial_limit),
            base_rtt: RwLock::new(Duration::from_millis(1000)),
            alpha: 2.0,
            beta: 4.0,
            min_limit: 1,
            max_limit: initial_limit * 10,
        }
    }

    pub fn with_alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn with_beta(mut self, beta: f64) -> Self {
        self.beta = beta;
        self
    }
}

impl LimitStrategy for VegasStrategy {
    fn current_limit(&self) -> usize {
        self.current_limit.load(Ordering::Relaxed)
    }

    fn on_success(&self, latency: Duration) {
        {
            let current_base = self.base_rtt.read();
            if latency < *current_base {
                drop(current_base);
                let mut write_base = self.base_rtt.write();
                if latency < *write_base {
                    *write_base = latency;
                }
            }
        }

        let base_rtt = *self.base_rtt.read();
        let limit = self.current_limit.load(Ordering::Relaxed);

        if base_rtt.as_nanos() == 0 || latency.as_nanos() == 0 {
            return; // Evita divisão por zero
        }

        let expected_throughput = limit as f64 / base_rtt.as_secs_f64();
        let actual_throughput = limit as f64 / latency.as_secs_f64();
        let diff = (expected_throughput - actual_throughput) * base_rtt.as_secs_f64();

        if diff > self.beta {
            if limit > self.min_limit {
                self.current_limit.fetch_sub(1, Ordering::Relaxed);
            }
        } else if diff < self.alpha && limit < self.max_limit {
            self.current_limit.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn on_error(&self) {
        let limit = self.current_limit.load(Ordering::Relaxed);
        if limit > self.min_limit {
            let new_limit = (limit * 3 / 4).max(self.min_limit);
            self.current_limit.store(new_limit, Ordering::Relaxed);
        }
    }
}
