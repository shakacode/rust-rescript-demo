use sqlx::Done;

use crate::{models::post::PostId, pg::PgPool};

pub async fn exec(id: PostId, db: &PgPool) -> sqlx::Result<u64> {
    sqlx::query_file!("src/pg/queries/post_delete.sql", id as _)
        .execute(db)
        .await
        .map(|res| res.rows_affected())
}
