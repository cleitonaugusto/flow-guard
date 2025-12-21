/* * Created by: Cleiton Augusto Correa Bezerra
 * Project: FlowGuard - Adaptive Backpressure for Rust
 * Algorithm: Optimized TCP Vegas for Concurrency Control
 */

use crate::LimitStrategy;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use parking_lot::RwLock;
use tracing; // Para o usuário ver a mágica acontecer

pub struct VegasStrategy {
    current_limit: AtomicUsize,
    base_rtt: RwLock<Duration>,
    alpha: usize,
    beta: usize,
    min_limit: usize,
    max_limit: usize,
}

impl VegasStrategy {
    pub fn new(initial_limit: usize, min_limit: usize, max_limit: usize) -> Self {
        Self {
            current_limit: AtomicUsize::new(initial_limit),
            base_rtt: RwLock::new(Duration::from_secs(3600)),
            alpha: 3,
            beta: 6,
            min_limit,
            max_limit,
        }
    }
}

impl LimitStrategy for VegasStrategy {
    fn current_limit(&self) -> usize {
        self.current_limit.load(Ordering::Relaxed)
    }

    fn on_success(&self, latency: Duration) {
        // 1. OTIMIZAÇÃO: Só atualiza o base_rtt se necessário (Read-first pattern)
        {
            let current_base = self.base_rtt.read();
            if latency < *current_base {
                drop(current_base); // Libera o lock de leitura antes de pedir o de escrita
                let mut write_base = self.base_rtt.write();
                if latency < *write_base {
                    *write_base = latency;
                    tracing::debug!(target: "flow_guard", "New Base RTT discovered: {:?}", latency);
                }
            }
        }

        let base_rtt = *self.base_rtt.read();
        let limit = self.current_limit.load(Ordering::Relaxed);

        // 2. CÁLCULO: diff = L * (1 - RTT_base / RTT_actual)
        let expected = (limit as f64 * base_rtt.as_secs_f64()) / latency.as_secs_f64();
        let diff = limit as f64 - expected;

        if diff < self.alpha as f64 {
            if limit < self.max_limit {
                let new_limit = self.current_limit.fetch_add(1, Ordering::Relaxed) + 1;
                tracing::trace!(target: "flow_guard", "Limit increased: {}", new_limit);
            }
        } else if diff > self.beta as f64 {
            if limit > self.min_limit {
                let new_limit = self.current_limit.fetch_sub(1, Ordering::Relaxed) - 1;
                tracing::warn!(target: "flow_guard", "Congestion detected! Limit decreased: {}", new_limit);
            }
        }
    }

    fn on_error(&self) {
        let limit = self.current_limit.load(Ordering::Relaxed);
        if limit > self.min_limit {
            let new_limit = (limit / 2).max(self.min_limit);
            self.current_limit.store(new_limit, Ordering::Relaxed);
            tracing::error!(target: "flow_guard", "Application error! Backing off to limit: {}", new_limit);
        }
    }
}