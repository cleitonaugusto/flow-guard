/* * Created and Developed by: Cleiton Augusto Correa Bezerra */
use axum::{response::{IntoResponse, Response}, http::StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlowError<E> {
    #[error("Request dropped due to high load")]
    Dropped,
    #[error("FlowGuard semaphore closed")]
    Closed,
    #[error("Application error: {0}")]
    AppError(#[from] E),
}

// Isso permite que o Axum transforme o erro em HTML/JSON automaticamente
impl<E: IntoResponse> IntoResponse for FlowError<E> {
    fn into_response(self) -> Response {
        match self {
            Self::Dropped => (StatusCode::SERVICE_UNAVAILABLE, "Service Overloaded - Try again later").into_response(),
            Self::Closed => (StatusCode::INTERNAL_SERVER_ERROR, "FlowGuard Closed").into_response(),
            Self::AppError(e) => e.into_response(),
        }
    }
}