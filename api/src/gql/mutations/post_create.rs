use graphql::Context as GqlContext;

use crate::{
    gql::{inputs::CreatePostInput, GqlError},
    models::post::Post,
    pg::queries as db,
};

pub async fn exec(input: CreatePostInput, ctx: &GqlContext<'_>) -> Result<Post, GqlError> {
    let res = db::post_create::exec(input, db!(ctx)?).await;

    match res {
        Ok(data) => Ok(data),
        Err(error) => {
            error!(error);
            Err(GqlError::InternalServerError)
        }
    }
}
