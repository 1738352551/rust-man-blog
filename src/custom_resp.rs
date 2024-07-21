use actix_web::body::EitherBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonResp<T> {
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> JsonResp<T> {
    /// 成功的响应
    pub fn success(msg: &str, data: Option<T>) -> Self {
        JsonResp {
            code: StatusCode::OK.as_u16() as i32,
            msg: msg.to_string(),
            data,
        }
    }

    /// 失败的响应
    pub fn fail(code: i32, msg: &str) -> Self {
        JsonResp {
            code,
            msg: "".to_string(),
            data: None,
        }
    }
}

impl<T: Serialize> Responder for JsonResp<T> {
    type Body = EitherBody<String>;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(err) => return HttpResponse::from_error(err).map_into_right_body(),
        };

        match HttpResponse::Ok()
            .content_type(ContentType::json())
            .message_body(body)
        {
            Ok(res) => res.map_into_left_body(),
            Err(err) => HttpResponse::from_error(err).map_into_right_body(),
        }
    }
}
