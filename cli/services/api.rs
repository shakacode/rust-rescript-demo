use crate::{env, Cmd, Dir, Process, CFG};

pub fn build() -> Cmd {
    Cmd {
        run: "cargo build --package=api".to_string(),
        env: env::empty(),
        dir: Dir::Root,
        msg: "Building API",
    }
}

pub fn clean() -> Cmd {
    Cmd {
        run: "cargo clean --package=api".to_string(),
        env: env::empty(),
        dir: Dir::Root,
        msg: "Cleaning API",
    }
}

pub fn up() -> Process {
    Process::new(
        "api",
        Cmd {
            run: "cargo run --package=api --color=always".to_string(),
            env: CFG.env(),
            dir: Dir::Root,
            msg: "Running API server",
        },
    )
}

pub fn watch() -> Process {
    Process::new(
        "api",
        Cmd {
            run: "cargo watch --watch api --exec 'run --package=api --color=always'".to_string(),
            env: CFG.env(),
            dir: Dir::Root,
            msg: "Running reloadable API server",
        },
    )
}
