/* * Created and Developed by: Cleiton Augusto Correa Bezerra */

use axum::response::IntoResponse;

#[tokio::main]
#[cfg(all(feature = "axum", feature = "tower"))]
async fn main() {
    use axum::{error_handling::HandleErrorLayer, routing::get, Router};
    use flow_guard::{FlowError, FlowGuardLayer, VegasStrategy};
    use tower::ServiceBuilder;

    let strategy = VegasStrategy::new(50);
    let flow_layer = FlowGuardLayer::new(strategy);

    let app = Router::new()
        .route("/", get(|| async { "Hello, FlowGuard!" }))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(
                    |err: FlowError<std::convert::Infallible>| async move { err.into_response() },
                ))
                .layer(flow_layer),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("🚀 Servidor rodando em http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(all(feature = "axum", feature = "tower")))]
fn main() {
    println!("Este exemplo requer as features 'axum' e 'tower'");
    println!("Execute com: cargo run --example server_demo --features axum,tower");
}
