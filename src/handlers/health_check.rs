use sqlx::postgres::PgPool;
use warp::{http::StatusCode, reject, Rejection, Reply};

pub async fn health_check(db_pool: PgPool) -> Result<impl Reply, Rejection> {
    let result = sqlx::query!("SELECT * FROM blank")
        .fetch_one(&db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            reject()
        })?;
    tracing::info!("{:?}", result);

    Ok(StatusCode::OK)
}
