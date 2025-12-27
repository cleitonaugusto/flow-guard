/* * Created and Developed by: Cleiton Augusto Correa Bezerra
 */

use crate::error::FlowError;
use crate::LimitStrategy;
use std::sync::Arc;
use std::time::Instant;

use crate::semaphore::DynamicSemaphore;

#[derive(Clone)]
pub struct FlowGuard<S: LimitStrategy> {
    strategy: Arc<S>,
    semaphore: Arc<DynamicSemaphore>,
}

impl<S: LimitStrategy + 'static> FlowGuard<S> {
    pub fn new(strategy: S) -> Self {
        let initial_limit = strategy.current_limit();
        Self {
            strategy: Arc::new(strategy),
            semaphore: Arc::new(DynamicSemaphore::new(initial_limit)),
        }
    }

    pub async fn run<F, T, E>(&self, f: F) -> Result<T, FlowError<E>>
    where
        F: std::future::Future<Output = Result<T, E>>,
    {
        // 1. Tenta adquirir permissão (Backpressure dinâmico)
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| FlowError::Dropped)?;

        let start = Instant::now();

        // 2. Executa a tarefa do usuário
        let result = f.await;

        let duration = start.elapsed();

        // 3. Informa a estratégia sobre o sucesso ou falha
        match &result {
            Ok(_) => self.strategy.on_success(duration),
            Err(_) => self.strategy.on_error(),
        }

        // 4. ATUALIZAÇÃO CRÍTICA: Atualiza o semáforo com o novo limite
        let new_limit = self.strategy.current_limit();
        self.semaphore.set_limit(new_limit);

        // 5. Retorna o resultado
        result.map_err(FlowError::AppError)
    }

    // Métodos para observabilidade
    pub fn current_limit(&self) -> usize {
        self.strategy.current_limit()
    }

    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }
}
