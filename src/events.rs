use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

pub async fn create_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

pub async fn select_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::OK, Json(res))
}

pub async fn update_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

pub async fn delete_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}
