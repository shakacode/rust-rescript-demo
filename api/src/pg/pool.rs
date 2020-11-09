use sqlx::postgres::PgPool;

use crate::env;

pub async fn new() -> PgPool {
    let addr = format!(
        "postgres://{user}:{password}@{host}:{port}/{db}",
        user = env::pg_user(),
        password = env::pg_password(),
        host = env::pg_host(),
        port = env::pg_port(),
        db = env::pg_database()
    );

    PgPool::connect(&addr)
        .await
        .expect("Failed to initialize PG pool")
}
