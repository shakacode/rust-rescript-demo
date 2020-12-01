use crate::{services::docker, Cmd, Dir, Env, EnvData, Exec, Result, TcpAddr, CFG};

pub fn create_database(env: &Env) -> Cmd {
    Cmd {
        run: "sqlx database create".to_string(),
        env: EnvData::one("DATABASE_URL", CFG.pg_url(env)),
        dir: Dir::Root,
        msg: match env {
            Env::Dev => "Creating development database",
            Env::Prod => "Creating production database",
            Env::Test => "Creating test database",
        },
    }
}

pub fn drop_database(env: &Env) -> Cmd {
    Cmd {
        run: "sqlx database drop -y".to_string(),
        env: EnvData::one("DATABASE_URL", CFG.pg_url(env)),
        dir: Dir::Root,
        msg: match env {
            Env::Dev => "Dropping development database",
            Env::Prod => "Dropping production database",
            Env::Test => "Dropping test database",
        },
    }
}

pub fn prepare_database_schema(env: &Env) -> Cmd {
    Cmd {
        run: "cargo sqlx prepare".to_string(),
        env: EnvData::one("DATABASE_URL", CFG.pg_url(env)),
        dir: Dir::Api,
        msg: match env {
            Env::Dev => "Preparing schema against development database",
            Env::Prod => "Preparing schema against production database",
            Env::Test => "Preparing schema against test database",
        },
    }
}

pub fn create_migration(name: String) -> Cmd {
    Cmd {
        run: format!("sqlx migrate add {}", name),
        env: EnvData::one("DATABASE_URL", CFG.pg_url(&Env::Dev)),
        dir: Dir::Api,
        msg: "Creating migration",
    }
}

pub fn run_migrations(env: &Env) -> Cmd {
    Cmd {
        run: "sqlx migrate run".to_string(),
        env: EnvData::one("DATABASE_URL", CFG.pg_url(env)),
        dir: Dir::Api,
        msg: match env {
            Env::Dev => "Running migrations against development database",
            Env::Prod => "Running migrations against production database",
            Env::Test => "Running migrations against test database",
        },
    }
}

pub async fn run_one_off_cmds_against_db(cmds: Vec<Cmd>) -> Result {
    let pg_status = docker::compose::pg_status().await?;

    if let docker::compose::ServiceStatus::Stopped = pg_status {
        let pg = TcpAddr {
            host: CFG.pg_host(&Env::Dev),
            port: CFG.pg_port(&Env::Dev),
        };
        Exec::cmd(docker::compose::start_detached_pg()).await?;
        pg.wait().await?;
    }

    Exec::cmd_seq(cmds).await?;

    if let docker::compose::ServiceStatus::Stopped = pg_status {
        Exec::cmd(docker::compose::stop_pg()).await?;
    }

    Ok(())
}
