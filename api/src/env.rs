use std::env;

pub fn web_host() -> String {
    env::var("WEB_HOST").expect("WEB_HOST is not set")
}

pub fn web_port() -> String {
    env::var("WEB_PORT").expect("WEB_PORT is not set")
}

pub fn api_host() -> String {
    env::var("API_HOST").expect("API_HOST is not set")
}

pub fn api_port() -> String {
    env::var("API_PORT").expect("API_PORT is not set")
}

pub fn api_graphql_path() -> String {
    env::var("API_GRAPHQL_PATH").expect("API_GRAPHQL_PATH is not set")
}

pub fn api_health_path() -> String {
    env::var("API_HEALTH_PATH").expect("API_HEALTH_PATH is not set")
}

pub fn pg_host() -> String {
    env::var("PG_HOST").expect("PG_HOST is not set")
}

pub fn pg_port() -> String {
    env::var("PG_PORT").expect("PG_PORT is not set")
}

pub fn pg_user() -> String {
    env::var("PG_USER").expect("PG_USER is not set")
}

pub fn pg_password() -> String {
    env::var("PG_PASSWORD").expect("PG_PASSWORD is not set")
}

pub fn pg_database() -> String {
    env::var("PG_DATABASE").expect("PG_DATABASE is not set")
}
