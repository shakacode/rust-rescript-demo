use graphql::Context as GqlContext;
use serde::Serialize;
use sqlx::Error as SqlxError;

use crate::{
    gql::{inputs::UpdatePostInput, GqlError},
    models::post::Post,
    pg::queries as db,
};

#[derive(Serialize, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    PostNotFound,
}

pub async fn exec(input: UpdatePostInput, ctx: &GqlContext<'_>) -> Result<Post, GqlError<Error>> {
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
