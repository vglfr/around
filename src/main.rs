use axum::Router;
use axum::routing::{delete, get, post, put};
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {
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
        .nest("/v0/users", users);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_user() {
    println!("create_user");
}

async fn select_user() {
    println!("select_user");
}

async fn update_user() {
    println!("update_user");
}

async fn delete_user() {
    println!("delete_user");
}

async fn create_events() {
    println!("create_events");
}

async fn select_events() {
    println!("select_events");
}

async fn update_events() {
    println!("update_events");
}

async fn delete_events() {
    println!("delete_events");
}
