use std::future::Future;

use axum::extract::{FromRequest, FromRequestParts, Path, Request, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use deadpool_diesel::postgres::Pool;
use deadpool_diesel::InteractError;
use diesel::result::Error;
use diesel::{AsChangeset, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use fake::Dummy;
use fake::faker::name::en::Name;
use fake::faker::company::en::CompanyName;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::users::{self, dsl};
use crate::common::{ApiError, RequestBody, ResponseBody};

type UserResponse = (StatusCode, Json<ResponseBody<User>>);

#[derive(AsChangeset, Deserialize, Dummy, Identifiable, Insertable, Queryable, Selectable, Serialize, ToSchema)]
#[diesel(primary_key(id))]
pub struct User {
    #[dummy(faker = "0..")]
    id: i32,
    #[dummy(faker = "Name()")]
    name: String,
    #[dummy(faker = "32")]
    fingerprint: String,
    #[dummy(faker = "-12..12")]
    timezone_offset: Option<i32>,
    #[dummy(faker = "CompanyName()")]
    favorite_team: Option<String>,
    dark_mode: Option<bool>,
}

impl<S> FromRequest<S> for User
where
    S: Send + Sync,
{
    type Rejection = UserResponse;
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

pub struct UserId(i32);

impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
{
    type Rejection = UserResponse;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Path::<i32>::from_request_parts(parts, state).await {
            Ok(value) => Ok(UserId(value.0)),
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

#[utoipa::path(post, path = "/v0/user", responses((status = 202, body = ResponseBody<User>)))]
pub async fn create_user(pool: State<Pool>, user: User) -> UserResponse {
    let connection = pool.get().await.unwrap();

    let result = connection.interact(|cursor| {
        diesel::insert_into(dsl::users)
            .values(user)
            .on_conflict_do_nothing()
            .get_result::<User>(cursor)
    });

    handle_result(result).await
}

#[utoipa::path(get, path = "/v0/user/{user_id}", responses((status = 200, body = ResponseBody<User>)))]
pub async fn select_user(user_id: UserId, pool: State<Pool>) -> UserResponse {
    let connection = pool.get().await.unwrap();

    let result = connection.interact(move |cursor|
        dsl::users
            .find(user_id.0)
            .select(User::as_select())
            .get_result(cursor)
    );

    handle_result(result).await
}

#[utoipa::path(put, path = "/v0/user", responses((status = 202, body = ResponseBody<User>)))]
pub async fn update_user(pool: State<Pool>, user: User) -> UserResponse {
    let connection = pool.get().await.unwrap();

    let result = connection.interact(move |cursor| {
        diesel::update(users::table)
            .set(&user)
            .get_result::<User>(cursor)
    });

    handle_result(result).await
}

#[utoipa::path(delete, path = "/v0/user/{user_id}", responses((status = 202, body = ResponseBody<User>)))]
pub async fn delete_user(user_id: UserId, pool: State<Pool>) -> UserResponse {
    let connection = pool.get().await.unwrap();

    let result = connection.interact(move |cursor| {
        diesel::delete(dsl::users.find(user_id.0))
            .get_result::<User>(cursor)
    });

    handle_result(result).await
}

async fn handle_result<T>(result: T) -> UserResponse
where
    T: Future<Output = Result<Result<User, Error>, InteractError>>,
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
