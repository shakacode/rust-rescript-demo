use crate::{error, Cmd, Dir, EnvData, Error, Exec, File, Result};

fn check_node() -> Cmd {
    Cmd {
        run: "node --version".to_string(),
        env: EnvData::parent(),
        dir: Dir::Root,
        msg: "Checking Node",
    }
}

fn check_yarn() -> Cmd {
    Cmd {
        run: "yarn --version".to_string(),
        env: EnvData::parent(),
        dir: Dir::Root,
        msg: "Checking Yarn",
    }
}

fn check_cargo_watch() -> Cmd {
    Cmd {
        run: "cargo watch --version".to_string(),
        env: EnvData::parent(),
        dir: Dir::Root,
        msg: "Checking Cargo watch",
    }
}

fn copy_env_file() -> Cmd {
    Cmd {
        run: format!(
            "cp {} {}",
            File::EnvExample.file_name(),
            File::Env.file_name()
        ),
        env: EnvData::parent(),
        dir: Dir::Env,
        msg: "Copying base env file",
    }
}

fn copy_dev_env_file() -> Cmd {
    Cmd {
        run: format!(
            "cp {} {}",
            File::DevEnvExample.file_name(),
            File::DevEnv.file_name()
        ),
        env: EnvData::parent(),
        dir: Dir::Env,
        msg: "Copying development env file",
    }
}

fn copy_prod_env_file() -> Cmd {
    Cmd {
        run: format!(
            "cp {} {}",
            File::ProdEnvExample.file_name(),
            File::ProdEnv.file_name()
        ),
        env: EnvData::parent(),
        dir: Dir::Env,
        msg: "Copying production env file",
    }
}

fn copy_test_env_file() -> Cmd {
    Cmd {
        run: format!(
            "cp {} {}",
            File::TestEnvExample.file_name(),
            File::TestEnv.file_name()
        ),
        env: EnvData::parent(),
        dir: Dir::Env,
        msg: "Copying test env file",
    }
}

pub async fn ensure_prerequisites() -> Result {
    if let Err(_) = Exec::cmd(check_node()).await {
        return Err(Error::Io(error::other(
            "Node is not installed. Install it from https://nodejs.org",
        )));
    }
    if let Err(_) = Exec::cmd(check_yarn()).await {
        return Err(Error::Io(error::other(
            "Yarn is not installed. Install it from https://yarnpkg.com",
        )));
    }
    if let Err(_) = Exec::cmd(check_cargo_watch()).await {
        return Err(Error::Io(error::other(
            "Cargo watch is not installed. Install it from https://github.com/passcod/cargo-watch",
        )));
    }

    if !File::Env.exists() {
        Exec::cmd(copy_env_file()).await?;
    }
    if !File::DevEnv.exists() {
        Exec::cmd(copy_dev_env_file()).await?;
    }
    if !File::ProdEnv.exists() {
        Exec::cmd(copy_prod_env_file()).await?;
    }
    if !File::TestEnv.exists() {
        Exec::cmd(copy_test_env_file()).await?;
    }

    Ok(())
}
