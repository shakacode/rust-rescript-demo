#[macro_export]
macro_rules! db {
    ($ctx:expr) => {{
        $ctx.data::<actix_web::web::Data<sqlx::postgres::PgPool>>()
            .map_err(|error| {
                error!(format!(
                    "Failed to get DB connection from GQL context: {}",
                    error.message
                ));
                GqlError::InternalServerError
            })
    }};
}
