/* * Created and Developed by: Cleiton Augusto Correa Bezerra */

use axum::{routing::get, Router, error_handling::HandleErrorLayer};
use flow_guard::{FlowGuardLayer, VegasStrategy, FlowError};
use tower::ServiceBuilder;

// Handler simples para o teste
async fn handler() -> &'static str {
    "Acesso autorizado pelo FlowGuard!"
}

#[tokio::main]
async fn main() {
    // 1. Inicializa a estratégia corretamente
    let strategy = VegasStrategy::new(50, 10, 500);

    // 2. USA A LAYER DA SUA LIB (Não use struct local aqui!)
    let flow_layer = FlowGuardLayer::new(strategy);

    // 3. Aplica ao seu Router com tratamento de erro
    let app = Router::new()
        .route("/api/data", get(handler))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: FlowError<std::convert::Infallible>| async move {
                    axum::response::IntoResponse::into_response(err)
                }))
                .layer(flow_layer)
        );

    // 4. Servidor moderno (TcpListener)
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Servidor 2 (API) rodando em http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}