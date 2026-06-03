use sqlx::{Pool, Postgres};

pub async fn get_entries(db: &Pool<Postgres>) -> Result<Vec<String>, sqlx::Error> {
    let entries = sqlx::query_scalar::<_, String>("SELECT * FROM public.entries")
        .fetch_all(db)
        .await?;

    Ok(entries)
}
