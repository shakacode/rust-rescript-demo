use actix_cors::Cors;
use actix_web::{
    guard::{Get, Post},
    http::header,
    web, App, HttpResponse, HttpServer,
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
        let web_app = format!(
            "http://{host}:{port}",
            host = env::web_host(),
            port = env::web_port()
        );
        let cors = Cors::default()
            .allowed_origin(&web_app)
            .allowed_methods(vec!["POST"])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        let health_path = &env::api_health_path();
        let gql_path = &env::api_graphql_path();

        let app = App::new()
            .data(pg.clone())
            .data(gql.clone())
            .wrap(cors)
            .route(health_path, web::get().to(|| HttpResponse::NoContent()))
            .service(
                web::resource(gql_path)
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
