use crate::{
    gql::inputs::CreatePostInput,
    models::post::{Post, PostId},
    pg::PgPool,
};

pub async fn exec(input: CreatePostInput, db: &PgPool) -> sqlx::Result<Post> {
    let title = &input.title;
    let content = &input.content;

    sqlx::query_file_as!(Post, "src/pg/queries/post_create.sql", title, content)
        .fetch_one(db)
        .await
}
