use graphql::Context as GqlContext;
use sqlx::Error as SqlxError;

use crate::{
    gql::{inputs::UpdatePostInput, GqlError, GqlResult},
    models::post::Post,
    pg::queries as db,
};

gql_error!(
    pub enum Error {
        PostNotFound,
    }
);

pub async fn exec(input: UpdatePostInput, ctx: &GqlContext<'_>) -> GqlResult<Post, Error> {
    let res = db::post_update::exec(input, db!(ctx)?).await;

    match res {
        Ok(data) => Ok(data),
        Err(SqlxError::RowNotFound) => Err(GqlError::Extended(Error::PostNotFound)),
        Err(error) => {
            error!(error);
            Err(GqlError::InternalServerError)
        }
    }
}
