use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::model{User, UserInfo};

pub async fn list_users(service: Extension<UserService>) ->Result<Json<Vec<User>>,StatusCode> {
  match service.list_users().await {
    Ok(users) => Ok(Json(users)),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}

pub async fn get_user_by_id(service:Extension<UserService>,Path(id):Path<i32>) -> Result<Json<User>,StatusCode> {
  match service.get_users_by_id(id).await {
    Ok(user) => Ok(Json(user)),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
  
}

pub async fn create_user(service:Extension<UserServce>,Json(user):Json<UserInfo>) -> StatusCode {
    match service.create_user(user).await {
        Ok(_) => StatusCode::Ok,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  
}

pub async fn update_user(service:Extension<UserService>, Path(id):Path<i32>, Json(user):Json<UserInfo>) -> StatusCode {
    match service.update_user(id, user).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  
}

pub async fn delete_user(service:Extension<UserService>, Path(id):Path<i32>) -> StatusCode {
    match service.delete_user(id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  
}
