use flow_guard::{FlowGuard, VegasStrategy};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn debug_semaphore() {
    println!("🔍 Debugando semáforo...");

    let strategy = Arc::new(VegasStrategy::new(2)); // Limite baixo
    let guard = FlowGuard::new(Arc::clone(&strategy));

    println!("1. Estado inicial:");
    println!("   Limite: {}", guard.current_limit());
    println!("   Permissões: {}", guard.available_permits());

    // Tenta adquirir uma permissão
    println!("2. Tentando acquire...");
    let result = guard
        .run(async {
            println!("   Dentro da task...");
            sleep(Duration::from_millis(50)).await;
            Ok::<_, &str>("task 1")
        })
        .await;

    match result {
        Ok(msg) => println!("3. ✅ Task 1 completada: {}", msg),
        Err(e) => println!("3. ❌ Task 1 falhou: {:?}", e),
    }

    println!("4. Estado após task:");
    println!("   Limite: {}", guard.current_limit());
    println!("   Permissões: {}", guard.available_permits());

    // Tenta outra task
    println!("5. Tentando segunda task...");
    let result = guard
        .run(async {
            println!("   Dentro da task 2...");
            sleep(Duration::from_millis(50)).await;
            Ok::<_, &str>("task 2")
        })
        .await;

    match result {
        Ok(msg) => println!("6. ✅ Task 2 completada: {}", msg),
        Err(e) => println!("6. ❌ Task 2 falhou: {:?}", e),
    }

    println!("🎉 Debug completo!");
}
