/* * Created and Developed by: Cleiton Augusto Correa Bezerra
 * Exemplo de uso real do FlowGuard com Axum 0.8
 */

use axum::{routing::get, Router, error_handling::HandleErrorLayer};
use flow_guard::{FlowGuardLayer, VegasStrategy, FlowError};
use axum::response::IntoResponse;

async fn handler() -> &'static str {
    "🛡️ FlowGuard de Cleiton Bezerra: Protegido!"
}

#[tokio::main]
async fn main() {
    let strategy = VegasStrategy::new(10);
    let flow_layer = FlowGuardLayer::new(strategy);

    let app = Router::new()
        .route("/api/data", get(handler))
        .layer(
            // No Axum 0.8, usamos ServiceBuilder para lidar com erros de Middleware
            tower::ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: FlowError<std::convert::Infallible>| async move {
                    // Transforma o erro da lib numa resposta amigável
                    err.into_response()
                }))
                .layer(flow_layer)
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 FlowGuard rodando em http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}