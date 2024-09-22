use axum::{extract::{Path, Query, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use serde_json::json;

use crate::{database::{DbPool, RouterType}, errors::errors::AppError, models::show::NewShow, repositories::show_repository::{self, create_show, get_all_shows, get_show_by_id}};

pub fn get_shows_router() -> RouterType {
    let router = Router::new()
        .route("/", get(index))
        .route("/:id", get(show))
        .route("/", post(create))
        .route("/seed", post(seed))
        .route("/count", get(count));

    router
}

#[derive(serde::Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i64>,
}

async fn index(
    State(pool): State<DbPool>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let shows = get_all_shows(pool, page, per_page).await?;

    let response = json!({
        "count": shows.len(),
        "data": shows,
    });
    Ok(Json(response))
}

async fn show(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let show = get_show_by_id(pool, id).await?;

    Ok(Json(show))
}

async fn create(
    State(pool): State<DbPool>,
    Json(show): Json<NewShow>,
) -> Result<impl IntoResponse, AppError> {
    let show = create_show(pool, show).await?;

    Ok(Json(show))
}

#[derive(serde::Deserialize)]
struct Count {
    count: Option<u64>,
}

async fn seed(
    State(pool): State<DbPool>,
    Query(count): Query<Count>,
) -> Result<impl IntoResponse, AppError> {
    let count = count.count.unwrap_or(1000);
    let count = show_repository::seed(pool, count).await?;

    Ok(Json(json!({"message": format!("Seeded database with {count} shows!")})))
}

async fn count(
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse, AppError> {
    let count = show_repository::count(pool).await?;

    Ok(Json(json!({"count": count})))
}
