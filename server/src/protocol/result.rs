use warp::reject::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;
