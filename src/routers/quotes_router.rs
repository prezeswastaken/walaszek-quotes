use axum::{extract::{Path, Query, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use serde_json::json;

use crate::{database::{DbPool, RouterType}, errors::errors::AppError, models::quote::NewQuote, repositories::{self, quote_repository::{self, create_quote, get_all_quotes, get_quote_by_id}}, resources::quote_resource::QuoteResource};

pub fn get_quotes_router() -> RouterType {
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
    let quotes = get_all_quotes(pool, page, per_page).await?;

    let resources = quotes.clone()
        .into_iter()
        .map(|(quote, character)| QuoteResource::make(quote, character))
        .collect::<Vec<QuoteResource>>();

    let response = json!({
        "count": resources.len(),
        "data": resources,
    });

    Ok(Json(response))
}

async fn show(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let quote = get_quote_by_id(pool.clone(), id).await?;
    let character = repositories::character_repository::get_character_by_id(pool, quote.character_id).await?;

    let resource = QuoteResource::make(quote, character);

    Ok(Json(resource))
}

async fn create(
    State(pool): State<DbPool>,
    Json(quote): Json<NewQuote>,
) -> Result<impl IntoResponse, AppError> {
    let quote = create_quote(pool.clone(), quote).await?;
    let character = repositories::character_repository::get_character_by_id(pool, quote.character_id).await?;

    let resource = QuoteResource::make(quote, character);

    Ok(Json(resource))
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
    let count = quote_repository::seed(pool, count).await?;

    Ok(Json(json!({"message": format!("Seeded database with {count} quotes!")})))
}

async fn count(
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse, AppError> {
    let count = quote_repository::count(pool).await?;

    Ok(Json(json!({"count": count})))
}
