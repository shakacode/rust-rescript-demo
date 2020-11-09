use crate::{
    gql::inputs::UpdatePostInput,
    models::post::{Post, PostId},
    pg::PgPool,
};

pub async fn exec(input: UpdatePostInput, db: &PgPool) -> sqlx::Result<Post> {
    let id = &input.id;
    let title = &input.title;
    let content = &input.content;

    sqlx::query_file_as!(
        Post,
        "src/pg/queries/post_update.sql",
        id as _,
        title,
        content
    )
    .fetch_one(db)
    .await
}
