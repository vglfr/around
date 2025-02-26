use std::future::Future;

use axum::extract::{FromRequest, Request, State};
use axum::http::StatusCode;
use axum::Json;
use deadpool_diesel::postgres::Pool;
use deadpool_diesel::InteractError;
use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use fake::Dummy;
use fake::faker::name::en::Name;
use fake::faker::company::en::CompanyName;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::users::{self, dsl};
use crate::common::{ApiError, RequestBody, ResponseBody};

type UserResponse = (StatusCode, Json<ResponseBody<User>>);

#[derive(Clone, Debug, Deserialize, Dummy, Insertable, Serialize, Queryable, Selectable, ToSchema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

#[utoipa::path(post, path = "/v0/user", responses((status = 202, body = ResponseBody<User>)))]
pub async fn create_user(pool: State<Pool>, user: User) -> UserResponse {
    let connection = pool.get().await.unwrap();

    let command = connection.interact(|cursor| {
        diesel::insert_into(dsl::users)
            .values(user)
            .get_result::<User>(cursor)
    });

    handle_result(command).await
}

#[utoipa::path(get, path = "/v0/user", responses((status = 200, body = ResponseBody<Vec<User>>)))]
pub async fn select_user(pool: State<Pool>) -> (StatusCode, Json<ResponseBody<Vec<User>>>) {
    let connection = pool.get().await.unwrap();

    let data = connection.interact(|cursor|
        dsl::users
            .select(User::as_select())
            .load(cursor)
            .unwrap()
    ).await.unwrap();

    let res = ResponseBody::ResponseOk { data, links: "links".to_string() };
    (StatusCode::OK, Json(res))
}

#[utoipa::path(put, path = "/v0/user", responses((status = 202, body = ResponseBody<User>)))]
pub async fn update_user(pool: State<Pool>, user: User) -> UserResponse {
    let connection = pool.get().await.unwrap();

    let command = connection.interact(move |cursor| {
        diesel::update(dsl::users.find(user.id))
            .set((
                dsl::name.eq(user.name),
                dsl::fingerprint.eq(user.fingerprint),
                dsl::timezone_offset.eq(user.timezone_offset),
                dsl::favorite_team.eq(user.favorite_team),
                dsl::dark_mode.eq(user.dark_mode),
            ))
            .get_result::<User>(cursor)
    });

    handle_result(command).await
}

#[utoipa::path(delete, path = "/v0/user", responses((status = 202, body = ResponseBody<User>)))]
pub async fn delete_user(pool: State<Pool>, user: User) -> UserResponse {
    let connection = pool.get().await.unwrap();

    let command = connection.interact(move |cursor| {
        diesel::delete(dsl::users.find(user.id))
            .get_result::<User>(cursor)
    });

    handle_result(command).await
}

async fn handle_result<T>(command: T) -> UserResponse
where
    T: Future<Output = Result<Result<User, Error>, InteractError>>,
{
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
