#[tokio::test]
async fn test_dynamic_simple() {
    println!("🧪 Teste SIMPLIFICADO do semáforo dinâmico");

    use flow_guard::{FlowGuard, VegasStrategy};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;

    // Configuração mais simples
    let strategy = Arc::new(VegasStrategy::new(2)); // Limite BAIXO
    let guard = FlowGuard::new(Arc::clone(&strategy));

    println!("1. Estado inicial:");
    println!("   Limite: {}", guard.current_limit());
    println!("   Permissões: {}", guard.available_permits());

    // Executa apenas 2 tarefas (dentro do limite)
    println!("2. Executando 2 tarefas...");

    let results = tokio::join!(
        guard.run(async {
            sleep(Duration::from_millis(50)).await;
            Ok::<_, &str>("task1")
        }),
        guard.run(async {
            sleep(Duration::from_millis(50)).await;
            Ok::<_, &str>("task2")
        })
    );

    println!("3. Resultados: {:?}", results);

    // Ambas devem ter sucesso
    assert!(results.0.is_ok());
    assert!(results.1.is_ok());

    println!("4. Estado final:");
    println!("   Limite: {}", guard.current_limit());
    println!("   Permissões: {}", guard.available_permits());

    println!("✅ Teste simplificado passou!");
}
