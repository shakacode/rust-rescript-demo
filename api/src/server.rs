use actix_web::{
    guard::{Get, Post},
    web, App, HttpServer,
};

use crate::{env, gql, pg};

pub async fn run() -> std::io::Result<()> {
    let pg = pg::pool::new().await;
    let gql = gql::schema::new();

    let addr = format!(
        "{host}:{port}",
        host = env::api_host(),
        port = env::api_port()
    );

    HttpServer::new(move || {
        let app = App::new().data(pg.clone()).data(gql.clone()).service(
            web::resource("/api")
                .guard(Post())
                .to(gql::http::api::endpoint),
        );

        #[cfg(debug_assertions)]
        return app.service(
            web::resource("/")
                .guard(Get())
                .to(gql::http::playground::endpoint),
        );

        #[cfg(not(debug_assertions))]
        return app;
    })
    .bind(&addr)?
    .run()
    .await
}
