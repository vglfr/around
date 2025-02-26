use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, TimeZone, Utc};
use deadpool_diesel::postgres::Pool;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use fake::Dummy;
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::company::en::Buzzword;
use serde_json::Value;

use crate::schema::events::{self, dsl};

// type EventResponse = (StatusCode, Json<ResponseBody<Event>>);
// type EventsResponse = (StatusCode, Json<ResponseBody<Vec<Event>>>);

#[derive(Debug, Dummy, Insertable, Queryable, Selectable)]
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

pub async fn create_event() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

pub async fn create_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

pub async fn select_event() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::OK, Json(res))
}

pub async fn select_events(pool: State<Pool>) -> (StatusCode, Json<Value>) {
    let connection = pool.get().await.unwrap();

    let data = connection.interact(|cursor|
        dsl::events
            .select(Event::as_select())
            .load(cursor)
            .unwrap()
    ).await.unwrap();

    dbg!(&data);

    let res = serde_json::json!({ "data": 42 });
    (StatusCode::OK, Json(res))
}

pub async fn update_event() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

pub async fn update_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

pub async fn delete_event() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}

pub async fn delete_events() -> (StatusCode, Json<Value>) {
    let res = serde_json::json!({ "data": 42 });
    (StatusCode::ACCEPTED, Json(res))
}
