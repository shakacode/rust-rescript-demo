use graphql::{Context, EmptySubscription, FieldResult, Schema};

use crate::{
    gql::{
        inputs::{CreatePostInput, UpdatePostInput},
        mutations, queries, GqlError, GqlOk,
    },
    models::post::{Post, PostId},
};

pub type GqlSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn new() -> GqlSchema {
    Schema::build(Query, Mutation, EmptySubscription).finish()
}

pub struct Query;
pub struct Mutation;

#[graphql::Object]
impl Query {
    async fn posts(&self, ctx: &Context<'_>) -> FieldResult<Vec<Post>> {
        queries::post_get_all::exec(ctx)
            .await
            .map_err(GqlError::into)
    }
    async fn post(&self, ctx: &Context<'_>, id: PostId) -> FieldResult<Post> {
        queries::post_get_by_id::exec(id, ctx)
            .await
            .map_err(GqlError::into)
    }
}

#[graphql::Object]
impl Mutation {
    async fn create_post(&self, ctx: &Context<'_>, input: CreatePostInput) -> FieldResult<Post> {
        mutations::post_create::exec(input, ctx)
            .await
            .map_err(GqlError::into)
    }
    async fn update_post(&self, ctx: &Context<'_>, input: UpdatePostInput) -> FieldResult<Post> {
        mutations::post_update::exec(input, ctx)
            .await
            .map_err(GqlError::into)
    }
    async fn delete_post(&self, ctx: &Context<'_>, id: PostId) -> FieldResult<GqlOk> {
        mutations::post_delete::exec(id, ctx)
            .await
            .map_err(GqlError::into)
    }
}
