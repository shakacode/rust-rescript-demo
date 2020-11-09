#[derive(graphql::InputObject)]
pub struct CreatePostInput {
    pub title: String,
    pub content: String,
}
