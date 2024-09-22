use warp::{reject::reject, Filter};

pub fn auth_filter() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("Authorization")
        .and_then(|auth_header: Option<String>| async move {
            match auth_header {
                Some(auth_value) if auth_value == "Bearer jebac wilkolaki" => Ok(()),
                _ => Err(reject()),
            }
        })
        .untuple_one()
}
