use sqlx::{Pool, Postgres};

use crate::repositories::entry_repository;

pub async fn get_entries(
    db: &Pool<Postgres>,
) -> Result<Vec<String>, sqlx::Error> {

    entry_repository::get_entries(db).await
}