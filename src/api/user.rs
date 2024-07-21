use std::borrow::Borrow;

use actix_web::{web, Responder};
use sea_orm::EntityTrait;

use crate::{custom_resp::JsonResp, error::ServerError, model::blog::{Entity as Blog, Model}, AppState};

/// path: /users/login
pub async fn login(data: web::Data<AppState>) -> Result<impl Responder, ServerError> {
    let db = data.db.lock().await;
    let blogs = Blog::find().all(&*db).await;
    Ok(JsonResp::<Vec<Model>>::success("登录成功!", blogs.ok()))
}
