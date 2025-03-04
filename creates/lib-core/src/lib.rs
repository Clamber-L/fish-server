mod database;
mod error;
mod jwt;
mod middleware;

use axum::extract::{Path, Query};
use axum::Json;
use axum_extra::extract::WithRejection;
pub use database::*;
pub use error::*;
pub use jwt::*;
use lib_utils::HttpResult;
pub use middleware::*;

pub type JsonParam<T> = WithRejection<Json<T>, AppError>;
pub type PathParam<T> = WithRejection<Path<T>, AppError>;
pub type QueryParam<T> = WithRejection<Query<T>, AppError>;

pub type ApiResult<T> = Result<HttpResult<T>, AppError>;

pub fn generate_snowflake_id() -> Result<String, AppError> {
    Ok(sonyflake::Sonyflake::new()?.next_id()?.to_string())
}
