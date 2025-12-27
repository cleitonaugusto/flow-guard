/* * Created and Developed by: Cleiton Augusto Correa Bezerra
 * Project: FlowGuard - Adaptive Concurrency Control & Backpressure
 * License: MIT / Apache-2.0
 */

//! # FlowGuard
//!
//! `flow-guard` é uma biblioteca de controle de concorrência adaptativo.
//! Desenvolvida por Cleiton Augusto Correa Bezerra, ela implementa algoritmos
//! de backpressure dinâmico para proteger sistemas de alta carga.

// 1. Declaração dos módulos internos
pub mod error;
pub mod limiter;
mod semaphore;
pub mod strategy;

#[cfg(feature = "tower")]
pub mod integration;

pub use error::FlowError;
pub use limiter::FlowGuard;
pub use strategy::VegasStrategy;

#[cfg(feature = "tower")]
pub use integration::FlowGuardLayer;

use std::time::Duration;

/// Trait fundamental para definir como o limite de requisições deve se comportar.
///
/// Implementado por estratégias como `VegasStrategy`.
pub trait LimitStrategy: Send + Sync {
    /// Retorna o limite de concorrência atual permitido pela estratégia.
    fn current_limit(&self) -> usize;

    /// Chamado após uma execução bem-sucedida para atualizar a latência.
    fn on_success(&self, latency: Duration);

    /// Chamado quando ocorre um erro para que a estratégia possa reduzir a carga.
    fn on_error(&self);
}

impl<S: LimitStrategy + ?Sized> LimitStrategy for std::sync::Arc<S> {
    fn current_limit(&self) -> usize {
        (**self).current_limit()
    }
    fn on_success(&self, latency: std::time::Duration) {
        (**self).on_success(latency)
    }
    fn on_error(&self) {
        (**self).on_error()
    }
}
