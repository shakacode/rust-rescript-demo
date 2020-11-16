use graphql::Context as GqlContext;

use crate::{
    gql::{inputs::CreatePostInput, GqlError, GqlResult},
    models::post::Post,
    pg::queries as db,
};

pub async fn exec(input: CreatePostInput, ctx: &GqlContext<'_>) -> GqlResult<Post> {
    let res = db::post_create::exec(input, db!(ctx)?).await;

    match res {
        Ok(data) => Ok(data),
        Err(error) => {
            error!(error);
            Err(GqlError::InternalServerError)
        }
    }
}
