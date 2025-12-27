/* * Created and Developed by: Cleiton Augusto Correa Bezerra
 * Semaphore dinâmico para FlowGuard - VERSÃO FINAL CORRIGIDA
 */

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;

#[derive(Debug)]
pub struct DynamicSemaphore {
    max_permits: AtomicUsize,
    available_permits: AtomicUsize,
    notify: Notify,
}

impl DynamicSemaphore {
    pub fn new(initial_permits: usize) -> Self {
        Self {
            max_permits: AtomicUsize::new(initial_permits),
            available_permits: AtomicUsize::new(initial_permits),
            notify: Notify::new(),
        }
    }

    pub fn set_limit(&self, new_limit: usize) {
        let old_limit = self.max_permits.swap(new_limit, Ordering::SeqCst);

        if new_limit > old_limit {
            // Adiciona novas permissões
            let diff = new_limit - old_limit;
            let old_available = self.available_permits.fetch_add(diff, Ordering::SeqCst);

            // Se havia waiters bloqueados, notifica alguns
            if old_available == 0 && diff > 0 {
                // Notifica no máximo diff waiters
                for _ in 0..diff.min(128) {
                    // Limite razoável
                    self.notify.notify_one();
                }
            }
        }
        // Para diminuir: permissões extras serão consumidas naturalmente
    }

    pub async fn acquire(&self) -> Result<DynamicPermit, ()> {
        loop {
            // Tenta adquirir
            if let Some(permit) = self.try_acquire() {
                return Ok(permit);
            }

            // Espera por notificação
            self.notify.notified().await;

            // IMPORTANTE: Após acordar, deve tentar novamente
            // Outra task pode ter pegado a permissão antes
        }
    }

    #[allow(dead_code)]
    pub fn try_acquire(&self) -> Option<DynamicPermit> {
        let mut current = self.available_permits.load(Ordering::SeqCst);

        loop {
            if current == 0 {
                return None;
            }

            match self.available_permits.compare_exchange_weak(
                current,
                current - 1,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    return Some(DynamicPermit {
                        semaphore: Arc::new(self.clone()),
                    })
                }
                Err(actual) => current = actual,
            }
        }
    }

    pub fn available_permits(&self) -> usize {
        self.available_permits.load(Ordering::Relaxed)
    }

    #[allow(dead_code)]
    pub fn current_limit(&self) -> usize {
        self.max_permits.load(Ordering::Relaxed)
    }

    fn release(&self) {
        let old = self.available_permits.fetch_add(1, Ordering::SeqCst);

        // Se estava em 0, agora tem 1 permissão - notifica UM waiter
        if old == 0 {
            self.notify.notify_one();
        }

        // Garante que não exceda o limite máximo
        let current = self.available_permits.load(Ordering::Relaxed);
        let max = self.max_permits.load(Ordering::Relaxed);
        if current > max {
            // Se excedeu (devido a race condition), ajusta
            self.available_permits.store(max, Ordering::Relaxed);
        }
    }
}

impl Clone for DynamicSemaphore {
    fn clone(&self) -> Self {
        Self {
            max_permits: AtomicUsize::new(self.max_permits.load(Ordering::Relaxed)),
            available_permits: AtomicUsize::new(self.available_permits.load(Ordering::Relaxed)),
            notify: Notify::new(),
        }
    }
}

pub struct DynamicPermit {
    semaphore: Arc<DynamicSemaphore>,
}

impl Drop for DynamicPermit {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}
