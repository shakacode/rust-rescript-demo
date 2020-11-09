use sqlx::types::Uuid;

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize, Debug)]
#[sqlx(transparent)]
pub struct PostId(Uuid);

graphql::scalar!(PostId);

#[derive(serde::Serialize, sqlx::FromRow, graphql::SimpleObject, Debug)]
pub struct Post {
    pub id: PostId,
    pub title: String,
    pub content: String,
}
