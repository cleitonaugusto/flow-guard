/* * Created and Developed by: Cleiton Augusto Correa Bezerra */

use axum::{error_handling::HandleErrorLayer, routing::get, Router};
use flow_guard::{FlowError, FlowGuardLayer, VegasStrategy}; // Importe FlowError
use std::net::SocketAddr;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    // 1. Estratégia Vegas (Inicia em 50, min 10, max 500)
    let strategy = VegasStrategy::new(50);

    // 2. Criamos o Layer
    let flow_layer = FlowGuardLayer::new(strategy);

    // 3. Router configurado para 2025 (Axum 0.8)
    let app = Router::new()
        .route(
            "/",
            get(|| async { "Hello, Cleiton! FlowGuard está ativo." }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(
                    |err: FlowError<std::convert::Infallible>| async move {
                        // Se o FlowGuard barrar a requisição, ele retorna 503 automaticamente
                        axum::response::IntoResponse::into_response(err)
                    },
                ))
                .layer(flow_layer), // Passamos por valor, não por referência
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 Servidor 1 rodando em http://{}", addr);

    // No Axum 0.8, usamos axum::serve
    axum::serve(listener, app).await.unwrap();
}
