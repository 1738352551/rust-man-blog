use actix_web::Responder;

use crate::{error::ServerError, custom_resp::JsonResp};

/// path: /api/users/login
pub async fn login() -> Result<impl Responder, ServerError> {

    Ok(
        JsonResp::<String> {
            code: 200,
            msg: "123123".to_string(),
            data: None,
        }
    )
}