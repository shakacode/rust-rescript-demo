use crate::{Cmd, Dir, Process, CFG};

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
