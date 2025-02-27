use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(untagged)]
pub enum ResponseBody<T> {
    ResponseOk {
        data: T,
        links: Links,
    },
    ResponseErr {
        errors: Vec<ApiError>,
    },
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RequestBody<T> {
    pub data: T,
}

#[derive(Serialize, ToSchema)]
pub struct ApiError {
    pub status: String,
    pub detail: String,
}

#[derive(Serialize, ToSchema)]
pub struct Links {
    pub this: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

pub async fn error_routing() -> (StatusCode, Json<ResponseBody<()>>) {
    let errors = vec![
        ApiError { status: "404".to_string(), detail: "path not found".to_string() }
    ];
    let res = ResponseBody::ResponseErr { errors };
    (StatusCode::NOT_FOUND, Json(res))
}
