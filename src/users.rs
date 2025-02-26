use axum::extract::{FromRequest, Request, State};
use axum::http::StatusCode;
use axum::Json;
use deadpool_diesel::postgres::Pool;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::common::{ApiError, RequestBody, ResponseBody};

#[derive(Clone, Debug, Deserialize, Insertable, Serialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
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

pub async fn create_user(pool: State<Pool>, user: User) -> (StatusCode, Json<ResponseBody<User>>) {
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

pub async fn select_user(pool: State<Pool>) -> (StatusCode, Json<ResponseBody<Vec<User>>>) {
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

pub async fn update_user(pool: State<Pool>, user: User) -> (StatusCode, Json<ResponseBody<User>>) {
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

pub async fn delete_user(pool: State<Pool>, user: User) -> (StatusCode, Json<ResponseBody<User>>) {
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
