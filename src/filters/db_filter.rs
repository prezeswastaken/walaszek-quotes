use warp::Filter;

use crate::database::DB;

pub fn db_filter(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
