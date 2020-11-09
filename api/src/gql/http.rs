pub mod api {
    use actix_web::web::Data;
    use graphql_actix_web::{Request as GqlRequest, Response as GqlResponse};

    use crate::{gql::schema::GqlSchema, pg::PgPool};

    pub async fn endpoint(
        pg: Data<PgPool>,
        schema: Data<GqlSchema>,
        req: GqlRequest,
    ) -> GqlResponse {
        schema.execute(req.into_inner().data(pg)).await.into()
    }
}

#[cfg(debug_assertions)]
pub mod playground {
    use actix_web::{HttpResponse, Result};
    use graphql::http::{playground_source, GraphQLPlaygroundConfig};

    pub async fn endpoint() -> Result<HttpResponse> {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(playground_source(GraphQLPlaygroundConfig::new("/api"))))
    }
}
