use axum::Json;
use utoipa::OpenApi;

use crate::{events, users};

#[derive(OpenApi)]
#[openapi(paths(
    openapi,
    events::create_events,
    events::select_events,
    events::update_events,
    events::delete_events,
    users::create_user,
    users::select_user,
    users::update_user,
    users::delete_user,
))]
struct ApiDoc;

#[utoipa::path(get, path = "/v0/docs", responses((status = 200, body = ())))]
pub async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
