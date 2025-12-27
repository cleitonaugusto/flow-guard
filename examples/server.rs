/* * Created and Developed by: Cleiton Augusto Correa Bezerra */

use axum::{error_handling::HandleErrorLayer, routing::get, Router};
use flow_guard::{FlowError, FlowGuardLayer, VegasStrategy};
use tower::ServiceBuilder;


async fn handler() -> &'static str {
    "Acesso autorizado pelo FlowGuard!"
}

#[tokio::main]
async fn main() {
    let strategy = VegasStrategy::new(50);

    let flow_layer = FlowGuardLayer::new(strategy);

    let app = Router::new().route("/api/data", get(handler)).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(
                |err: FlowError<std::convert::Infallible>| async move {
                    axum::response::IntoResponse::into_response(err)
                },
            ))
            .layer(flow_layer),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Servidor 2 (API) rodando em http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
