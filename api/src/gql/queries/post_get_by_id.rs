use graphql::Context as GqlContext;
use sqlx::Error as SqlxError;

use crate::{
    gql::{GqlError, GqlResult},
    models::post::{Post, PostId},
    pg::queries as db,
};

gql_error!(
    pub enum Error {
        PostNotFound,
    }
);

pub async fn exec(id: PostId, ctx: &GqlContext<'_>) -> GqlResult<Post, Error> {
    let res = db::post_get_by_id::exec(id, db!(ctx)?).await;

    match res {
        Ok(post) => Ok(post),
        Err(SqlxError::RowNotFound) => Err(GqlError::Extended(Error::PostNotFound)),
        Err(error) => {
            error!(error);
            Err(GqlError::InternalServerError)
        }
    }
}
