use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ResponseBody<T> {
    ResponseOk {
        data: T,
        links: String,
    },
    ResponseErr {
        errors: Vec<ApiError>,
    },
}

#[derive(Debug, Deserialize)]
pub struct RequestBody<T> {
    pub data: T,
}

#[derive(Serialize)]
pub struct ApiError {
    pub status: String,
    pub detail: String,
}

// struct Links {
//     this: String,
//     next: Option<String>,
// }

pub async fn error_routing() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "error": 1 });
    (StatusCode::NOT_FOUND, Json(res))
}
