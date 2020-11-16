use graphql::Context as GqlContext;

use crate::{
    gql::{GqlError, GqlOk, GqlResult},
    models::post::PostId,
    pg::queries as db,
};

gql_error!(
    pub enum Error {
        PostNotFound,
    }
);

pub async fn exec(id: PostId, ctx: &GqlContext<'_>) -> GqlResult<GqlOk, Error> {
    let res = db::post_delete::exec(id, db!(ctx)?).await;

    match res {
        Ok(0) => Err(GqlError::Extended(Error::PostNotFound)),
        Ok(1) => Ok(GqlOk::new()),
        Ok(_) => {
            warn!("Somehow, we deleted more than one post");
            Ok(GqlOk::new())
        }
        Err(error) => {
            error!(error);
            Err(GqlError::InternalServerError)
        }
    }
}
