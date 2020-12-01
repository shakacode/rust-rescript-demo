use std::{collections::HashMap, iter::FromIterator};

use crate::{Env, EnvData, File, HttpAddr};

lazy_static! {
    pub static ref CFG: Cfg = Cfg::load();
}

pub struct Cfg {
    dev: EnvData,
    prod: EnvData,
    test: EnvData,
}

impl Cfg {
    pub fn load() -> Self {
        #[allow(deprecated)] // it was undeprecated
        let base = HashMap::from_iter(
            dotenv::from_path_iter(File::Env.loc())
                .unwrap()
                .map(Result::unwrap),
        );
        #[allow(deprecated)] // it was undeprecated
        let dev_overrides = dotenv::from_path_iter(File::DevEnv.loc())
            .unwrap()
            .map(Result::unwrap);
        #[allow(deprecated)] // it was undeprecated
        let prod_overrides = dotenv::from_path_iter(File::ProdEnv.loc())
            .unwrap()
            .map(Result::unwrap);
        #[allow(deprecated)] // it was undeprecated
        let test_overrides = dotenv::from_path_iter(File::TestEnv.loc())
            .unwrap()
            .map(Result::unwrap);

        // TODO: We don't want anything db related except db name to be env dependent
        //       since we run only one instance of Postgres via Docker Compose.
        //       So we should prevent overrides of PG_HOST, PG_PORT etc.

        let mut dev = base.clone();
        dev.extend(dev_overrides);
        let mut prod = base.clone();
        prod.extend(prod_overrides);
        let mut test = base.clone();
        test.extend(test_overrides);

        Self {
            dev: EnvData::new(dev),
            prod: EnvData::new(prod),
            test: EnvData::new(test),
        }
    }

    pub fn data(&self, env: &Env) -> &EnvData {
        match env {
            Env::Dev => &self.dev,
            Env::Prod => &self.prod,
            Env::Test => &self.test,
        }
    }

    pub fn env(&self, env: &Env) -> EnvData {
        self.data(env).to_owned()
    }

    pub fn web_host(&self, env: &Env) -> String {
        self.data(env)
            .get("WEB_HOST")
            .expect("Failed to get WEB_HOST")
            .to_string()
    }

    pub fn web_port(&self, env: &Env) -> String {
        self.data(env)
            .get("WEB_PORT")
            .expect("Failed to get WEB_PORT")
            .to_string()
    }

    pub fn api_host(&self, env: &Env) -> String {
        self.data(env)
            .get("API_HOST")
            .expect("Failed to get API_HOST")
            .to_string()
    }

    pub fn api_port(&self, env: &Env) -> String {
        self.data(env)
            .get("API_PORT")
            .expect("Failed to get API_PORT")
            .to_string()
    }

    pub fn api_graphql_path(&self, env: &Env) -> String {
        self.data(env)
            .get("API_GRAPHQL_PATH")
            .expect("Failed to get API_GRAPHQL_PATH")
            .to_string()
    }

    pub fn api_health_path(&self, env: &Env) -> String {
        self.data(env)
            .get("API_HEALTH_PATH")
            .expect("Failed to get API_HEALTH_PATH")
            .to_string()
    }

    pub fn api_graphql_url(&self, env: &Env) -> HttpAddr {
        HttpAddr {
            host: self.api_host(env),
            port: self.api_port(env),
            path: self.api_graphql_path(env),
        }
    }

    pub fn api_health_url(&self, env: &Env) -> HttpAddr {
        HttpAddr {
            host: self.api_host(env),
            port: self.api_port(env),
            path: self.api_health_path(env),
        }
    }

    pub fn pg_host(&self, env: &Env) -> String {
        self.data(env)
            .get("PG_HOST")
            .expect("Failed to get PG_HOST")
            .to_string()
    }

    pub fn pg_port(&self, env: &Env) -> String {
        self.data(env)
            .get("PG_PORT")
            .expect("Failed to get PG_PORT")
            .to_string()
    }

    pub fn pg_user(&self, env: &Env) -> String {
        self.data(env)
            .get("PG_USER")
            .expect("Failed to get PG_USER")
            .to_string()
    }

    pub fn pg_password(&self, env: &Env) -> String {
        self.data(env)
            .get("PG_PASSWORD")
            .expect("Failed to get PG_PASSWORD")
            .to_string()
    }

    pub fn pg_database(&self, env: &Env) -> String {
        self.data(env)
            .get("PG_DATABASE")
            .expect("Failed to get PG_DATABASE")
            .to_string()
    }

    pub fn pg_url(&self, env: &Env) -> String {
        format!(
            "postgres://{user}:{password}@{host}:{port}/{database}",
            user = self.pg_user(env),
            password = self.pg_password(env),
            host = self.pg_host(env),
            port = self.pg_port(env),
            database = self.pg_database(env),
        )
    }

    pub fn envs_with_unique_dbs(&self) -> Vec<Env> {
        let dev_db = self.pg_database(&Env::Dev);
        let prod_db = self.pg_database(&Env::Prod);
        let test_db = self.pg_database(&Env::Test);
        let mut envs = vec![Env::Dev];
        if prod_db != dev_db {
            envs.push(Env::Prod)
        }
        if test_db != dev_db && test_db != prod_db {
            envs.push(Env::Test)
        }
        envs
    }
}
