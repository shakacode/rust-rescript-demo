use crate::models::post::PostId;

#[derive(graphql::InputObject)]
pub struct UpdatePostInput {
    pub id: PostId,
    pub title: String,
    pub content: String,
}
