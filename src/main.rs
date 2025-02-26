// use std::any::Any;
// use std::error::Error;

use axum::extract::{FromRequest, Request, State};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{delete, get, post, put};
use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
// use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub mod schema;

use crate::schema::users;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().with_target(false).compact().init();

    let url = std::env::var("DATABASE_URL").unwrap();
    let manager = Manager::new(url, Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

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
        )
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
#[serde(untagged)]
enum ResponseBody<T> {
    ResponseOk {
        data: T,
        links: String,
    },
    ResponseErr {
        errors: Vec<ApiError>,
    },
}

#[derive(Debug, Deserialize)]
struct RequestBody<T> {
    data: T,
}

#[derive(Serialize)]
struct ApiError {
    status: String,
    detail: String,
}

// struct Links {
//     this: String,
//     next: Option<String>,
// }

#[derive(Clone, Debug, Deserialize, Insertable, Serialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: i32,
    name: String,
}

impl<S> FromRequest<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ResponseBody<User>>);
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<RequestBody<User>>::from_request(req, state).await {
            Ok(value) => Ok(value.0.data),
            Err(err) => {
                let errors = vec![
                    ApiError { status: err.status().as_str().to_owned(), detail: err.body_text() }
                ];
                let res = ResponseBody::ResponseErr { errors };
                Err((StatusCode::BAD_REQUEST, Json(res)))
            },
        }
    }
}

async fn create_user(pool: State<Pool>, user: User) -> (StatusCode, Json<ResponseBody<User>>) {
    let connection = pool.get().await.unwrap();

    let command = connection.interact(|cursor| {
        diesel::insert_into(users::dsl::users)
            .values(user)
            .get_result::<User>(cursor)
    });

    match command.await.unwrap() {
        Ok(data) => {
            let res = ResponseBody::ResponseOk { data, links: "links".to_string() };
            (StatusCode::ACCEPTED, Json(res))
        }
        Err(err) => {
            let errors = vec![
                ApiError { status: "400".to_string(), detail: err.to_string() },
            ];
            let res = ResponseBody::ResponseErr { errors };
            (StatusCode::BAD_REQUEST, Json(res))
        }
    }
}

async fn select_user(pool: State<Pool>) -> (StatusCode, Json<ResponseBody<Vec<User>>>) {
    let connection = pool.get().await.unwrap();

    let data = connection.interact(|cursor|
        users::dsl::users
            .select(User::as_select())
            .load(cursor)
            .unwrap()
    ).await.unwrap();

    let res = ResponseBody::ResponseOk { data, links: "links".to_string() };
    (StatusCode::OK, Json(res))
}

async fn update_user(pool: State<Pool>, user: User) -> (StatusCode, Json<ResponseBody<User>>) {
    let connection = pool.get().await.unwrap();

    let command = connection.interact(move |cursor| {
        diesel::update(users::dsl::users.find(user.id))
            .set((
                users::dsl::name.eq(user.name),
            ))
            .get_result::<User>(cursor)
    });

    match command.await.unwrap() {
        Ok(data) => {
            let res = ResponseBody::ResponseOk { data, links: "links".to_string() };
            (StatusCode::ACCEPTED, Json(res))
        }
        Err(err) => {
            let errors = vec![
                ApiError { status: "400".to_string(), detail: err.to_string() },
            ];
            let res = ResponseBody::ResponseErr { errors };
            (StatusCode::BAD_REQUEST, Json(res))
        }
    }
}

async fn delete_user(pool: State<Pool>, user: User) -> (StatusCode, Json<ResponseBody<User>>) {
    let connection = pool.get().await.unwrap();
    let data = user.clone();

    let command = connection.interact(move |cursor| {
        diesel::delete(users::dsl::users.find(user.id))
            .execute(cursor)
    });

    match command.await.unwrap() {
        Ok(_) => {
            let res = ResponseBody::ResponseOk { data, links: "links".to_string() };
            (StatusCode::ACCEPTED, Json(res))
        }
        Err(err) => {
            let errors = vec![
                ApiError { status: "400".to_string(), detail: err.to_string() },
            ];
            let res = ResponseBody::ResponseErr { errors };
            (StatusCode::BAD_REQUEST, Json(res))
        }
    }
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
