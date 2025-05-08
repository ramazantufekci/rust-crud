use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

pub async fn list_users() ->(StatusCode, Json<Value>) {
  
}

pub async fn get_user_by_id(Path(id):Path<u64>) -> (StatusCode,Json<Value>) {
  
}

pub async fn create_user(Json(user):Json<User>) -> StatusCode {
  
}

pub async fn update_user(Path(id):Path<u64>, Json(user):Json<User>) -> StatusCode {
  
}

pub async fn delete_user(Path(id):Path<u64>) -> StatusCode {
  
}
