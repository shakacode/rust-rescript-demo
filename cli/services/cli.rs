use crate::{env, Cmd, Dir};

pub enum ReleaseCtx {
    Install,
    Update,
}

pub fn release(ctx: ReleaseCtx) -> Cmd {
    Cmd {
        run: "cargo install --path .".to_string(),
        env: env::empty(),
        dir: Dir::Root,
        msg: match ctx {
            ReleaseCtx::Install => "Installing CLI",
            ReleaseCtx::Update => "Updating CLI",
        },
    }
}
