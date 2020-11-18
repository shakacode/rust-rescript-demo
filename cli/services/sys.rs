use crate::{env, error, Cmd, Dir, Error, Exec, Result};

fn check_node() -> Cmd {
    Cmd {
        run: "node --version".to_string(),
        env: env::parent(),
        dir: Dir::Root,
        msg: "Checking Node",
    }
}

fn check_yarn() -> Cmd {
    Cmd {
        run: "yarn --version".to_string(),
        env: env::parent(),
        dir: Dir::Root,
        msg: "Checking Yarn",
    }
}

fn check_cargo_watch() -> Cmd {
    Cmd {
        run: "cargo watch --version".to_string(),
        env: env::parent(),
        dir: Dir::Root,
        msg: "Checking Cargo watch",
    }
}

pub async fn check() -> Result {
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
    Ok(())
}
