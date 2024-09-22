use axum::{extract::{Path, Query, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use serde_json::json;

use crate::{database::{DbPool, RouterType}, errors::errors::AppError, models::character::NewCharacter, repositories::character_repository::{self, create_character, get_all_characters, get_character_by_id}};

pub fn get_characters_router() -> RouterType {
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
    let characters = get_all_characters(pool, page, per_page).await?;

    let response = json!({
        "count": characters.len(),
        "data": characters,
    });
    Ok(Json(response))
}

async fn show(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let character = get_character_by_id(pool, id).await?;

    Ok(Json(character))
}

async fn create(
    State(pool): State<DbPool>,
    Json(character): Json<NewCharacter>,
) -> Result<impl IntoResponse, AppError> {
    let character = create_character(pool, character).await?;

    Ok(Json(character))
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
    let count = character_repository::seed(pool, count).await?;

    Ok(Json(json!({"message": format!("Seeded database with {count} characters!")})))
}

async fn count(
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse, AppError> {
    let count = character_repository::count(pool).await?;

    Ok(Json(json!({"count": count})))
}
