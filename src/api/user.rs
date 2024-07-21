use actix_web::Responder;

use crate::{custom_resp::JsonResp, error::ServerError};

/// path: /users/login
pub async fn login() -> Result<impl Responder, ServerError> {
    Ok(JsonResp::<String>::success("登录成功!", None))
}
