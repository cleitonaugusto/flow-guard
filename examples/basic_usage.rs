/* * Created and Developed by: Cleiton Augusto Correa Bezerra */
use flow_guard::{FlowGuard, LimitStrategy, VegasStrategy};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // Cria a estratégia em um Arc para podermos acessá-la depois
    let strategy = Arc::new(VegasStrategy::new(5));

    // Cria o FlowGuard
    let flow_guard = FlowGuard::new(Arc::clone(&strategy));

    println!("🚀 FlowGuard básico funcionando!");
    println!(
        "Limite inicial: {} concorrências simultâneas",
        strategy.current_limit()
    );
    println!("Testando...\n");

    // Cria várias tarefas concorrentes
    let mut handles = vec![];

    for i in 0..10 {
        let guard = flow_guard.clone();

        let handle = tokio::spawn(async move {
            println!("[Tarefa {}] Tentando executar...", i);

            // Usa o FlowGuard para executar a tarefa
            let result = guard
                .run(async {
                    // Simula um trabalho que leva tempo
                    sleep(Duration::from_millis(500)).await;

                    // Simula um possível erro (apenas para demonstração)
                    if i == 3 {
                        Err("Erro simulado na tarefa 3")
                    } else {
                        Ok(format!("Tarefa {} concluída com sucesso!", i))
                    }
                })
                .await;

            match result {
                Ok(msg) => println!("[Tarefa {}] ✅ {}", i, msg),
                Err(err) => println!("[Tarefa {}] ❌ Erro: {}", i, err),
            }
        });

        handles.push(handle);

        // Pequena pausa entre o spawn das tarefas
        sleep(Duration::from_millis(50)).await;
    }

    // Aguarda todas as tarefas terminarem
    for handle in handles {
        handle.await.unwrap();
    }

    println!("\n✅ Todas as tarefas concluídas!");
    println!("📊 Limite final: {}", strategy.current_limit());
}
