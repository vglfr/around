use axum::Router;
use axum::routing::{delete, get, post, put};
use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use around::{common, events, openapi, users};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().with_target(false).compact().init();

    let url = std::env::var("DATABASE_URL").unwrap();
    let manager = Manager::new(url, Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    let users = Router::new()
        .route("/", post(users::create_user))
        .route("/{user_id}", get(users::select_user))
        .route("/", put(users::update_user))
        .route("/{user_id}", delete(users::delete_user));

    let events = Router::new()
        .route("/", post(events::create_events))
        .route("/", get(events::select_events))
        .route("/", put(events::update_events))
        .route("/", delete(events::delete_events));

    let openapi = Router::new()
        .route("/", get(openapi::openapi));

    let app = Router::new()
        .nest("/v0/events", events)
        .nest("/v0/users", users)
        .nest("/v0/docs", openapi)
        .fallback(common::error_routing)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
        )
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
