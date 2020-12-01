use crate::{Cmd, Dir, EnvData};

pub fn install() -> Cmd {
    Cmd {
        run: "yarn install".to_string(),
        env: EnvData::empty(),
        dir: Dir::Root,
        msg: "Installing Yarn dependencies",
    }
}

pub fn remove_root_node_modules() -> Cmd {
    Cmd {
        run: "rm -rf node_modules".to_string(),
        env: EnvData::empty(),
        dir: Dir::Root,
        msg: "Removing root node_modules",
    }
}

pub fn remove_client_node_modules() -> Cmd {
    Cmd {
        run: "rm -rf node_modules".to_string(),
        env: EnvData::empty(),
        dir: Dir::Client,
        msg: "Removing client node_modules",
    }
}
