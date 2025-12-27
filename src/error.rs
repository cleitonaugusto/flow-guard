/* * Created and Developed by: Cleiton Augusto Correa Bezerra */
#[cfg(feature = "axum")]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
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

#[cfg(feature = "axum")]
impl<E> IntoResponse for FlowError<E>
where
    E: std::fmt::Display,
{
    fn into_response(self) -> Response {
        match self {
            Self::Dropped => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service Overloaded - Try again later",
            )
                .into_response(),
            Self::Closed => (StatusCode::INTERNAL_SERVER_ERROR, "FlowGuard Closed").into_response(),
            Self::AppError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}
