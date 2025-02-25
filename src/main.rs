use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
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

#[derive(Debug, Deserialize)]
struct RequestBody<T> {
    data: T,
}

// struct Links {
//     this: String,
//     next: Option<String>,
// }

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
        match Json::<RequestBody<User>>::from_request(req, state).await {
            Ok(value) => Ok(value.0.data),
            Err(err) => {
                let res = serde_json::json!({
                    "errors": [{
                        "status": err.status().as_str(),
                        "detail": err.body_text(),
                    }],
                });
                Err((StatusCode::BAD_REQUEST, Json(res)))
            },
        }
    }
}

async fn create_user(user: User) -> (StatusCode, Json<ResponseBody<User>>) {
    // create
    let res = ResponseBody { data: user, links: "links".to_string() };
    (StatusCode::ACCEPTED, Json(res))
}

async fn select_user() -> (StatusCode, Json<ResponseBody<Vec<User>>>) {
    // select
    let users = vec![
        User { id: 0, name: "foo".to_string() },
        User { id: 2, name: "bar".to_string() },
    ];
    let res = ResponseBody { data: users, links: "links".to_string() };
    (StatusCode::OK, Json(res))
}

async fn update_user(user: User) -> (StatusCode, Json<ResponseBody<User>>) {
    // update
    let res = ResponseBody { data: user, links: "links".to_string() };
    (StatusCode::ACCEPTED, Json(res))
}

async fn delete_user(user: User) -> (StatusCode, Json<ResponseBody<User>>) {
    // delete
    let res = ResponseBody { data: user, links: "links".to_string() };
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
