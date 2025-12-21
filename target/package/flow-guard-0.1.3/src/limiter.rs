/* * Created and Developed by: Cleiton Augusto Correa Bezerra
 */

use crate::LimitStrategy;
use std::sync::Arc;
use tokio::sync::Semaphore;
use std::time::Instant;
use crate::error::FlowError; // Importando o erro que criamos acima

pub struct FlowGuard<S: LimitStrategy> {
    strategy: Arc<S>,
    semaphore: Arc<Semaphore>,
}

impl<S: LimitStrategy + 'static> FlowGuard<S> {
    pub fn new(strategy: S) -> Self {
        let initial_limit = strategy.current_limit();
        Self {
            strategy: Arc::new(strategy),
            // Iniciamos o semáforo com o limite definido pela estratégia
            semaphore: Arc::new(Semaphore::new(initial_limit)),
        }
    }

    pub async fn run<F, T, E>(&self, f: F) -> Result<T, FlowError<E>>
    where
        F: std::future::Future<Output = Result<T, E>>,
    {
        // 1. Tenta adquirir permissão (Backpressure dinâmico)
        let _permit = self.semaphore.acquire().await
            .map_err(|_| FlowError::Dropped)?;

        let start = Instant::now();

        // 2. Executa a tarefa do usuário
        let result = f.await;

        let duration = start.elapsed();

        // 3. Informa a estratégia sobre o sucesso ou falha para ajustar o limite
        match &result {
            Ok(_) => self.strategy.on_success(duration),
            Err(_) => self.strategy.on_error(),
        }

        // 4. Retorna o resultado mapeando o erro interno para FlowError::AppError
        result.map_err(FlowError::AppError)
    }
}