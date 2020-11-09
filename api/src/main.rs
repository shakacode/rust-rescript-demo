#[macro_use]
mod log;

mod env;
mod gql;
mod models;
mod pg;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::init();
    server::run().await
}
