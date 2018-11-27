use actix_web::{error::ResponseError, HttpResponse};

#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "Internal Server Error: {}", _0)]
    InternalServerError(String),

    #[fail(display = "NotFound: {}", _0)]
    NotFound(String),
}

/// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            },
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
        }
    }
}
