use axum::{http::StatusCode, response::IntoResponse};

pub struct ResponseError(Box<ResponseErrorKind>);

impl<E> From<E> for ResponseError
where
    E: Into<ResponseErrorKind>,
{
    fn from(err: E) -> Self {
        ResponseError(Box::new(err.into()))
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) enum ResponseErrorKind {}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        // For now, we return a generic 500 Internal Server Error for all response errors.
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {:?}", self.0),
        )
            .into_response()
    }
}
