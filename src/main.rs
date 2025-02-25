use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::{HeaderMap, StatusCode};
use axum::{Json, Router};
use axum::routing::{delete, get, post, put};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().with_target(false).compact().init();

    let users = Router::new()
        .route("/", post(create_user))
        .route("/", get(select_user))
        .route("/", put(update_user))
        .route("/", delete(delete_user));

    let events = Router::new()
        .route("/", post(create_events))
        .route("/", get(select_events))
        .route("/", put(update_events))
        .route("/", delete(delete_events));

    let app = Router::new()
        .nest("/v0/events", events)
        .nest("/v0/users", users)
        .fallback(error_routing)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
        );

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct ResponseBody<T> {
    data: T,
    links: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: u8,
    name: String,
}

impl<S> FromRequest<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<User>::from_request(req, state).await {
            Ok(value) => {
                dbg!(&value);
                Ok(value.0)
            },
            Err(err) => {
                dbg!(&err);
                let res = serde_json::json!({ "status": err.status().as_u16(), "body_text": err.body_text() });
                Err((StatusCode::BAD_REQUEST, Json(res)))
            },
        }
    }
}

async fn create_user(body: Result<Json<Value>, JsonRejection>) -> (StatusCode, Json<Value>) {
    match body {
        Ok(_) => {
            let res = serde_json::json!({ "data": 42 });
            (StatusCode::ACCEPTED, Json(res))
        }
        Err(_) => {
            let res = serde_json::json!({ "error": 21 });
            (StatusCode::BAD_REQUEST, Json(res))
        }
    }
}

async fn select_user() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::OK, Json(res))
}

async fn update_user(_user: User) -> (StatusCode, Json<ResponseBody<User>>) {
    dbg!(&_user);
    let user = User { id: 0, name: "foo".to_string() };
    let res = ResponseBody { data: user, links: "links".to_string() };
    (StatusCode::ACCEPTED, Json(res))
}

async fn delete_user() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

async fn create_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

async fn select_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::OK, Json(res))
}

async fn update_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

async fn delete_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

async fn error_routing() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "error": 1 });
    (StatusCode::NOT_FOUND, Json(res))
}
