use crate::{
    models::post::{Post, PostId},
    pg::PgPool,
};

pub async fn exec(id: PostId, db: &PgPool) -> sqlx::Result<Post> {
    sqlx::query_file_as!(Post, "src/pg/queries/post_get_by_id.sql", id as _)
        .fetch_one(db)
        .await
}
