/* * Created and Developed by: Cleiton Augusto Correa Bezerra
 * FlowGuard - Tower/Axum Integration Layer
 */

use crate::{FlowGuard, LimitStrategy};
use crate::error::FlowError;
use tower::{Layer, Service};
use std::task::{Context, Poll};
use std::sync::Arc;
use futures_util::future::BoxFuture;

// --- 1. A LAYER ---
pub struct FlowGuardLayer<S: LimitStrategy> {
    guard: Arc<FlowGuard<S>>,
}

// Implementação manual de Clone para não exigir que S seja Clone
impl<S: LimitStrategy> Clone for FlowGuardLayer<S> {
    fn clone(&self) -> Self {
        Self {
            guard: self.guard.clone(),
        }
    }
}

impl<S: LimitStrategy + 'static> FlowGuardLayer<S> {
    pub fn new(strategy: S) -> Self {
        Self {
            guard: Arc::new(FlowGuard::new(strategy)),
        }
    }
}

impl<S, L> Layer<S> for FlowGuardLayer<L>
where
    L: LimitStrategy + 'static,
{
    type Service = FlowGuardService<S, L>;

    fn layer(&self, inner: S) -> Self::Service {
        FlowGuardService {
            inner,
            guard: self.guard.clone(),
        }
    }
}

// --- 2. O SERVICE ---
pub struct FlowGuardService<S, L: LimitStrategy> {
    inner: S,
    guard: Arc<FlowGuard<L>>,
}

impl<S: Clone, L: LimitStrategy> Clone for FlowGuardService<S, L> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            guard: self.guard.clone(),
        }
    }
}

impl<S, L, Req> Service<Req> for FlowGuardService<S, L>
where
    S: Service<Req> + Clone + Send + 'static,
    S::Future: Send + 'static,
    L: LimitStrategy + 'static,
    Req: Send + 'static,
{
    type Response = S::Response;
    type Error = FlowError<S::Error>;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Mapeia o erro do serviço interno para o nosso sistema de erros do FlowGuard
        self.inner.poll_ready(cx).map_err(FlowError::AppError)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let mut inner = self.inner.clone();
        let guard = self.guard.clone();

        Box::pin(async move {
            // O FlowGuard decide se executa ou bloqueia (Backpressure dinâmico)
            guard.run(inner.call(req)).await
        })
    }
}