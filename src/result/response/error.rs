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
pub(crate) enum ResponseErrorKind {
    Validator(#[from] validator::ValidationErrors),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        match *self.0 {
            ResponseErrorKind::Validator(validation_errors) => {
                let body = format!("Validation Error: {}", validation_errors);
                (StatusCode::BAD_REQUEST, body).into_response()
            }
        }
    }
}
