use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, TimeZone, Utc};
use deadpool_diesel::postgres::Pool;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use fake::Dummy;
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::company::en::Buzzword;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::common::ResponseBody;
use crate::schema::events::{self, dsl};

// type EventResponse = (StatusCode, Json<ResponseBody<Event>>);
type EventsResponse = (StatusCode, Json<ResponseBody<Vec<Event>>>);

#[derive(Debug, Dummy, Insertable, Queryable, Selectable, Serialize, ToSchema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    #[dummy(faker = "DateTimeBetween(Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(), Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap())")]
    created_at: DateTime<Utc>,
    #[dummy(faker = "0..")]
    user_id: i32,
    #[dummy(faker = "Buzzword()")]
    kind: String,
    #[dummy(faker = "0.0..400.0")]
    x_ft: f64,
    #[dummy(faker = "0.0..100.0")]
    y_ft: f64,
    #[dummy(faker = "0.0..4.0")]
    duration_s: f64,
    #[dummy(faker = "0..5000")]
    impressions: i32,
}

pub async fn create_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

#[utoipa::path(get, path = "/v0/events", responses((status = 200, body = ResponseBody<Vec<Event>>)))]
pub async fn select_events(pool: State<Pool>) -> EventsResponse {
    let connection = pool.get().await.unwrap();

    let data = connection.interact(|cursor|
        dsl::events
            .select(Event::as_select())
            .limit(5)
            .load(cursor)
            .unwrap()
    ).await.unwrap();

    // handle_result(command).await
    // dbg!(&data);

    // let res = serde_json::json!({ "data": 42 });
    // (StatusCode::OK, Json(res))
    let res = ResponseBody::ResponseOk { data, links: "links".to_string() };
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

// async fn handle_result<T>(command: T) -> UserResponse
// where
//     T: Future<Output = Result<Result<User, Error>, InteractError>>,
// {
//     match command.await.unwrap() {
//         Ok(data) => {
//             let res = ResponseBody::ResponseOk { data, links: "links".to_string() };
//             (StatusCode::ACCEPTED, Json(res))
//         }
//         Err(err) => {
//             let errors = vec![
//                 ApiError { status: "400".to_string(), detail: err.to_string() },
//             ];
//             let res = ResponseBody::ResponseErr { errors };
//             (StatusCode::BAD_REQUEST, Json(res))
//         }
//     }
// }
