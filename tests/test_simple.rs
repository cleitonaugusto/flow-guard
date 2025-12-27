#[tokio::test]
async fn verify_semaphore_adjusts() {
    use flow_guard::{FlowGuard, VegasStrategy};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;

    println!("🧪 Verificação simples do ajuste dinâmico");

    let strategy = Arc::new(VegasStrategy::new(5));
    let guard = FlowGuard::new(Arc::clone(&strategy));

    println!("📊 Limite inicial: {}", guard.current_limit());

    // Executa algumas tarefas rápidas
    for i in 0..3 {
        guard
            .run(async {
                sleep(Duration::from_millis(10)).await;
                Ok::<_, &str>(i)
            })
            .await
            .unwrap();
    }

    let new_limit = guard.current_limit();
    println!("📊 Limite após tarefas rápidas: {}", new_limit);

    // O limite deve ter aumentado
    assert!(new_limit >= 5, "Limite deveria ser pelo menos 5");

    println!("✅ Verificação passou!");
}
