use crate::{env, services::docker, Cmd, Dir, Exec, Result, CFG};

pub fn create_database() -> Cmd {
    Cmd {
        run: "sqlx database create".to_string(),
        env: env::one("DATABASE_URL", CFG.pg_url()),
        dir: Dir::Root,
        msg: "Creating database",
    }
}

pub fn drop_database() -> Cmd {
    Cmd {
        run: "sqlx database drop -y".to_string(),
        env: env::one("DATABASE_URL", CFG.pg_url()),
        dir: Dir::Root,
        msg: "Dropping database",
    }
}

pub fn prepare_database() -> Cmd {
    Cmd {
        run: "cargo sqlx prepare".to_string(),
        env: env::one("DATABASE_URL", CFG.pg_url()),
        dir: Dir::Api,
        msg: "Preparing database",
    }
}

pub fn create_migration(name: String) -> Cmd {
    Cmd {
        run: format!("sqlx migrate add {}", name),
        env: env::one("DATABASE_URL", CFG.pg_url()),
        dir: Dir::Api,
        msg: "Creating migration",
    }
}

pub fn run_migrations() -> Cmd {
    Cmd {
        run: "sqlx migrate run".to_string(),
        env: env::one("DATABASE_URL", CFG.pg_url()),
        dir: Dir::Api,
        msg: "Running migrations",
    }
}

pub async fn run_one_off_cmds_against_db(cmds: Vec<Cmd>) -> Result {
    let pg_status = docker::compose::pg_status().await?;

    if let docker::compose::ServiceStatus::Stopped = pg_status {
        Exec::cmd(docker::compose::start_detached_pg()).await?;
    }

    Exec::cmd_seq(cmds).await?;

    if let docker::compose::ServiceStatus::Stopped = pg_status {
        Exec::cmd(docker::compose::stop_pg()).await?;
    }

    Ok(())
}
