use std::future::Future;

use axum::extract::{FromRequest, FromRequestParts, Query, Request, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, TimeZone, Utc};
use deadpool_diesel::postgres::Pool;
use deadpool_diesel::InteractError;
use diesel::result::Error;
use diesel::{AsChangeset, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use fake::Dummy;
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::company::en::Buzzword;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::{ApiError, RequestBody, ResponseBody};
use crate::schema::events::{self, dsl};

type EventsResponse = (StatusCode, Json<ResponseBody<Vec<Event>>>);

#[derive(AsChangeset, Debug, Deserialize, Dummy, Identifiable, Insertable, Queryable, Selectable, Serialize, ToSchema)]
#[diesel(primary_key(created_at))]
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

pub struct Events(Vec<Event>);

impl<S> FromRequest<S> for Events
where
    S: Send + Sync,
{
    type Rejection = EventsResponse;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<RequestBody<Vec<Event>>>::from_request(req, state).await {
            Ok(value) => Ok(Events(value.0.data)),
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

#[derive(Debug, Deserialize)]
pub struct EventQuery {
    limit: Option<u8>,
    offset: Option<u16>,
    start: Option<DateTime<Utc>>,
}

impl<S> FromRequestParts<S> for EventQuery
where
    S: Send + Sync,
{
    type Rejection = EventsResponse;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Query::<EventQuery>::from_request_parts(parts, state).await {
            Ok(value) => Ok(value.0),
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

#[utoipa::path(post, path = "/v0/events", responses((status = 202, body = ResponseBody<Vec<Event>>)))]
pub async fn create_events(pool: State<Pool>, events: Events) -> EventsResponse {
    let connection = pool.get().await.unwrap();

    let result = connection.interact(|cursor|
        diesel::insert_into(dsl::events)
            .values(events.0)
            .on_conflict_do_nothing()
            .get_results::<Event>(cursor)
    );

    handle_result(result).await
}

#[utoipa::path(get, path = "/v0/events", responses((status = 200, body = ResponseBody<Vec<Event>>)))]
pub async fn select_events(query: EventQuery, pool: State<Pool>) -> EventsResponse {
    let connection = pool.get().await.unwrap();

    let result = connection.interact(move |cursor|
        dsl::events
            .filter(dsl::created_at.gt(query.start.unwrap_or_else(Utc::now)))
            .order_by(dsl::created_at)
            .offset(query.offset.map_or(0, i64::from))
            .limit(query.limit.map_or(32, i64::from))
            .select(Event::as_select())
            .load(cursor)
    );

    handle_result(result).await
}

#[utoipa::path(put, path = "/v0/events", responses((status = 202, body = ResponseBody<Vec<Event>>)))]
pub async fn update_events(pool: State<Pool>, events: Events) -> EventsResponse {
    let connection = pool.get().await.unwrap();

    let result = connection.interact(|cursor| {
        let mut responses = Vec::new();

        for event in events.0 {
            let response = diesel::update(events::table)
                .set(&event)
                .get_result::<Event>(cursor);

            responses.push(response);
        }

        responses.into_iter().collect()
    });

    handle_result(result).await
}

#[utoipa::path(delete, path = "/v0/events", responses((status = 202, body = ResponseBody<Vec<Event>>)))]
pub async fn delete_events(pool: State<Pool>, events: Events) -> EventsResponse {
    let connection = pool.get().await.unwrap();
    let ids = events.0.iter().map(|x| x.created_at).collect::<Vec<DateTime<Utc>>>();

    let result = connection.interact(|cursor|
        diesel::delete(dsl::events.filter(dsl::created_at.eq_any(ids)))
            .get_results::<Event>(cursor)
    );

    handle_result(result).await
}

async fn handle_result<T>(result: T) -> EventsResponse
where
    T: Future<Output = Result<Result<Vec<Event>, Error>, InteractError>>,
{
    match result.await.unwrap() {
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
