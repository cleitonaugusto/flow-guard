/* Teste de integração real */
use flow_guard::{FlowGuard, VegasStrategy};
use std::thread::sleep;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_high_load_scenario() {
    // Simula carga alta
    let strategy = VegasStrategy::new(5);
    let guard = FlowGuard::new(strategy);

    let start = Instant::now();
    let mut successes = 0;
    let mut errors = 0;

    // Executa 100 requisições rapidamente
    for i in 0..100 {
        match guard
            .run(async {
                sleep(Duration::from_millis(10)).await;
                Ok(i)
            })
            .await
        {
            Ok(_) => successes += 1,
            Err(e) => {
                println!("Request {} failed: {:?}", i, e);
                errors += 1;
            }
        }

        if i % 20 == 0 {
            sleep(Duration::from_millis(5)).await; // Pequena pausa
        }
    }

    let duration = start.elapsed();

    println!("Resultados:");
    println!("  Sucessos: {}", successes);
    println!("  Erros: {}", errors);
    println!("  Tempo total: {:?}", duration);
    println!(
        "  Requisições/segundo: {:.2}",
        100.0 / duration.as_secs_f64()
    );

    // Verifica resultados básicos
    assert!(successes > 0, "Deve haver pelo menos alguns sucessos");
    assert!(errors < 100, "Nem todas devem falhar");
}

fn main() {}
