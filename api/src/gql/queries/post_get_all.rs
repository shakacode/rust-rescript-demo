use graphql::Context as GqlContext;

use crate::{
    gql::{GqlError, GqlResult},
    models::post::Post,
    pg::queries as db,
};

pub async fn exec(ctx: &GqlContext<'_>) -> GqlResult<Vec<Post>> {
    let res = db::post_get_all::exec(db!(ctx)?).await;

    match res {
        Ok(data) => Ok(data),
        Err(error) => {
            error!(error);
            Err(GqlError::InternalServerError)
        }
    }
}
