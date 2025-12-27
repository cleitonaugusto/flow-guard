/* * Created and Developed by: Cleiton Augusto Correa Bezerra
 * FlowGuard - Performance & Resilience Benchmark
 */

use axum::{error_handling::HandleErrorLayer, response::IntoResponse, routing::get, Router};
use criterion::{criterion_group, criterion_main, Criterion};
use flow_guard::{FlowError, FlowGuardLayer, VegasStrategy};
use std::time::Duration;
use tokio::runtime::Runtime;
use tower::ServiceBuilder;

// Simula um handler de base de dados que demora 10ms (ajustado para o bench não demorar horas)
async fn slow_handler() -> &'static str {
    tokio::time::sleep(Duration::from_millis(10)).await;
    "Data processed"
}

fn bench_flow_guard_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("flow_guard_high_concurrency_overhead", |b| {
        b.to_async(&rt).iter(|| async {
            // 1. Criamos a estratégia e a camada de proteção
            let strategy = VegasStrategy::new(10);
            let flow_layer = FlowGuardLayer::new(strategy);

            // 2. Setup do Router com tratamento de erro obrigatório para Axum 0.8
            let app = Router::new().route("/test", get(slow_handler)).layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(
                        |err: FlowError<std::convert::Infallible>| async move {
                            // Converte o erro Dropped em uma resposta HTTP no benchmark
                            err.into_response()
                        },
                    ))
                    .layer(flow_layer),
            );

            // 3. Simulação de carga: 50 pedidos disparados simultaneamente
            let mut futures = Vec::new();
            for _ in 0..50 {
                let service = app.clone();
                futures.push(async move {
                    use tower::ServiceExt;
                    let request = axum::http::Request::builder()
                        .uri("/test")
                        .body(axum::body::Body::empty())
                        .unwrap();

                    // Executa a requisição através de toda a stack de middleware
                    service.oneshot(request).await
                });
            }

            // Aguarda o processamento de todo o lote
            futures_util::future::join_all(futures).await;
        });
    });
}

criterion_group!(benches, bench_flow_guard_throughput);
criterion_main!(benches);
