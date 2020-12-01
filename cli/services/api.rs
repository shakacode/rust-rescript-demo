use crate::{Cmd, Dir, Env, EnvData, Process, CFG};

pub fn build_dev() -> Cmd {
    Cmd {
        run: "cargo build --package=api".to_string(),
        env: EnvData::empty(),
        dir: Dir::Root,
        msg: "Building API server",
    }
}

pub fn build_release() -> Cmd {
    Cmd {
        run: "cargo build --package=api --release".to_string(),
        env: EnvData::empty(),
        dir: Dir::Root,
        msg: "Building API server release",
    }
}

pub fn clean() -> Cmd {
    Cmd {
        run: "cargo clean --package=api".to_string(),
        env: EnvData::empty(),
        dir: Dir::Root,
        msg: "Cleaning API",
    }
}

pub fn run_dev() -> Process {
    Process::new(
        "api",
        Cmd {
            run: "cargo run --package=api --color=always".to_string(),
            env: CFG.env(&Env::Dev),
            dir: Dir::Root,
            msg: "Running development API server",
        },
    )
}

pub fn run_release() -> Process {
    Process::new(
        "api",
        Cmd {
            run: "cargo run --package=api --release --color=always".to_string(),
            env: CFG.env(&Env::Prod),
            dir: Dir::Root,
            msg: "Running release build of API server",
        },
    )
}

pub fn watch_dev() -> Process {
    Process::new(
        "api",
        Cmd {
            run: "cargo watch --watch api --exec 'run --package=api --color=always'".to_string(),
            env: CFG.env(&Env::Dev),
            dir: Dir::Root,
            msg: "Running reloadable development API server",
        },
    )
}

pub fn watch_release() -> Process {
    Process::new(
        "api",
        Cmd {
            run: "cargo watch --watch api --exec 'run --package=api --release --color=always'"
                .to_string(),
            env: CFG.env(&Env::Prod),
            dir: Dir::Root,
            msg: "Running reloadable release build of API server",
        },
    )
}
