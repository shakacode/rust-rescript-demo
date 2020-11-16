use std::path::PathBuf;

lazy_static! {
    static ref ROOT: Root = Root::new();
}

struct Root(PathBuf);

impl Root {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().expect("Failed to get current directory of the process");
        Self(Self::find(cwd))
    }

    fn find(dir: PathBuf) -> PathBuf {
        if dir.join(".env").exists() {
            dir
        } else {
            Root::find(
                dir.parent()
                    .expect("Failed to get parent directory during root search")
                    .to_path_buf(),
            )
        }
    }

    pub fn path(&self) -> PathBuf {
        self.0.to_owned()
    }
}

#[derive(Clone)]
pub enum Dir {
    Root,
    Api,
    Client,
}

impl Dir {
    pub fn loc(&self) -> PathBuf {
        match self {
            Dir::Root => ROOT.path(),
            Dir::Api => Dir::Root.loc().join("api"),
            Dir::Client => Dir::Root.loc().join("client"),
        }
    }

    pub fn display(&self) -> String {
        let parent = Dir::Root.loc().parent().unwrap().to_owned();
        self.loc()
            .strip_prefix(parent)
            .unwrap()
            .display()
            .to_string()
    }

    pub fn extend(&self, path: &[&str]) -> String {
        let mut loc = self.loc();
        loc.extend(path);
        loc.into_os_string().into_string().unwrap()
    }
}

pub enum File {
    DotEnv,
}

impl File {
    pub fn loc(&self) -> PathBuf {
        match self {
            File::DotEnv => Dir::Root.loc().join(".env"),
        }
    }
}
