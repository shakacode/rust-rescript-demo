use crate::{Cmd, Dir, EnvData};

pub enum ReleaseCtx {
    Install,
    Update,
}

pub fn release(ctx: ReleaseCtx) -> Cmd {
    Cmd {
        run: "cargo install --path .".to_string(),
        env: EnvData::empty(),
        dir: Dir::Root,
        msg: match ctx {
            ReleaseCtx::Install => "Installing CLI",
            ReleaseCtx::Update => "Updating CLI",
        },
    }
}
