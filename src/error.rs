use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "内部错误")]
    InternalError,

    #[display(fmt = "错误请求")]
    BadClientData,

    #[display(fmt = "请求超时")]
    Timeout,

    #[display(fmt = "参数校验错误: [{}] {}", field, error_message)]
    ValidationError {
        field: String,
        error_message: String,
    },

    // #[display(fmt = "sql语句错误")]
    // DbErr
}

impl error::ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::BadClientData => StatusCode::BAD_REQUEST,
            ServerError::Timeout => StatusCode::REQUEST_TIMEOUT,
            ServerError::ValidationError { .. } => StatusCode::from_u16(444).unwrap(),
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
