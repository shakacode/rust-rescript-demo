use std::{collections::HashMap, iter::FromIterator};

use crate::{Env, File, HttpAddr};

lazy_static! {
    pub static ref CFG: Cfg = Cfg::load();
}

pub struct Cfg(Env);

impl Cfg {
    pub fn load() -> Self {
        #[allow(deprecated)] // it was undeprecated
        let vars = dotenv::from_path_iter(File::DotEnv.loc())
            .unwrap()
            .map(Result::unwrap);
        Self(HashMap::from_iter(vars))
    }

    pub fn env(&self) -> Env {
        self.0.to_owned()
    }

    pub fn web_host(&self) -> String {
        self.0
            .get("WEB_HOST")
            .expect("Failed to get WEB_HOST")
            .to_string()
    }

    pub fn web_port(&self) -> String {
        self.0
            .get("WEB_PORT")
            .expect("Failed to get WEB_PORT")
            .to_string()
    }

    pub fn api_host(&self) -> String {
        self.0
            .get("API_HOST")
            .expect("Failed to get API_HOST")
            .to_string()
    }

    pub fn api_port(&self) -> String {
        self.0
            .get("API_PORT")
            .expect("Failed to get API_PORT")
            .to_string()
    }

    pub fn api_graphql_path(&self) -> String {
        self.0
            .get("API_GRAPHQL_PATH")
            .expect("Failed to get API_GRAPHQL_PATH")
            .to_string()
    }

    pub fn api_health_path(&self) -> String {
        self.0
            .get("API_HEALTH_PATH")
            .expect("Failed to get API_HEALTH_PATH")
            .to_string()
    }

    pub fn api_graphql_url(&self) -> HttpAddr {
        HttpAddr {
            host: self.api_host(),
            port: self.api_port(),
            path: self.api_graphql_path(),
        }
    }

    pub fn api_health_url(&self) -> HttpAddr {
        HttpAddr {
            host: self.api_host(),
            port: self.api_port(),
            path: self.api_health_path(),
        }
    }

    pub fn pg_host(&self) -> String {
        self.0
            .get("PG_HOST")
            .expect("Failed to get PG_HOST")
            .to_string()
    }

    pub fn pg_port(&self) -> String {
        self.0
            .get("PG_PORT")
            .expect("Failed to get PG_PORT")
            .to_string()
    }

    pub fn pg_user(&self) -> String {
        self.0
            .get("PG_USER")
            .expect("Failed to get PG_USER")
            .to_string()
    }

    pub fn pg_password(&self) -> String {
        self.0
            .get("PG_PASSWORD")
            .expect("Failed to get PG_PASSWORD")
            .to_string()
    }

    pub fn pg_database(&self) -> String {
        self.0
            .get("PG_DATABASE")
            .expect("Failed to get PG_DATABASE")
            .to_string()
    }

    pub fn pg_url(&self) -> String {
        format!(
            "postgres://{user}:{password}@{host}:{port}/{database}",
            user = self.pg_user(),
            password = self.pg_password(),
            host = self.pg_host(),
            port = self.pg_port(),
            database = self.pg_database(),
        )
    }
}
