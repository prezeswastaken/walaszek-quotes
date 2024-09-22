use std::sync::Arc;
use warp::{Filter, Reply};

use diesel::SqliteConnection;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{
    database::DB,
    filters::db_filter::db_filter,
    models::show::NewShow,
    repositories::show_repository::{create_show, get_all_shows, get_show_by_id},
};

use super::auth_filter::auth_filter;

pub fn shows_filter(db: DB) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let base = warp::path("shows");

    let index = base
        .and(warp::get())
        .and(db_filter(db.clone()))
        .and(warp::path::end())
        .and_then(index);

    let create = base
        .and(warp::post())
        .and(auth_filter())
        .and(db_filter(db.clone()))
        .and(warp::body::json())
        .and_then(create);

    let show_by_id = base
        .and(warp::get())
        .and(db_filter(db.clone()))
        .and(warp::path::param())
        .and_then(show_by_id);

    index.or(create).or(show_by_id)
}

async fn index(db: DB) -> Result<impl Reply, warp::Rejection> {
    let shows = get_all_shows(db).await.map_err(|_| warp::reject())?;
    Ok(warp::reply::json(&json!(shows)))
}

async fn create(db: DB, show: NewShow) -> Result<impl Reply, warp::Rejection> {
    let show = create_show(db, show).await.map_err(|_| warp::reject())?;
    Ok(warp::reply::with_status(
        warp::reply::json(&show),
        warp::http::StatusCode::CREATED,
    ))
}

async fn show_by_id(db: DB, id: i32) -> Result<impl Reply, warp::Rejection> {
    let show = get_show_by_id(db, id).await.map_err(|_| warp::reject())?;
    Ok(warp::reply::json(&json!(show)))
}
