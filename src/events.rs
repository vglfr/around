use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

pub struct Event {
    created_at: i32,
    user: String,
    r#type: EventType,
    x_ft: f32,
    y_ft: f32,
    duration_s: f32,
    impressions: i32,
}

enum EventType {
    Firework,
    Mascot,
    Goal,
    Penalty,
}

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
