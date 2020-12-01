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
        if dir.join("Cargo.lock").exists() {
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
    Env,
    Api,
    Client,
    ClientCfg,
}

impl Dir {
    pub fn loc(&self) -> PathBuf {
        match self {
            Dir::Root => ROOT.path(),
            Dir::Env => Dir::Root.loc().join("env"),
            Dir::Api => Dir::Root.loc().join("api"),
            Dir::Client => Dir::Root.loc().join("client"),
            Dir::ClientCfg => Dir::Client.loc().join("cfg"),
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
    Env,
    EnvExample,
    DevEnv,
    DevEnvExample,
    ProdEnv,
    ProdEnvExample,
    TestEnv,
    TestEnvExample,
    WebpackDevConfig,
    WebpackProdConfig,
}

impl File {
    pub fn loc(&self) -> PathBuf {
        match self {
            File::Env => Dir::Env.loc().join(".env"),
            File::EnvExample => Dir::Env.loc().join("env.example"),
            File::DevEnv => Dir::Env.loc().join(".env.development"),
            File::DevEnvExample => Dir::Env.loc().join("env.development.example"),
            File::ProdEnv => Dir::Env.loc().join(".env.production"),
            File::ProdEnvExample => Dir::Env.loc().join("env.production.example"),
            File::TestEnv => Dir::Env.loc().join(".env.test"),
            File::TestEnvExample => Dir::Env.loc().join("env.test.example"),
            File::WebpackDevConfig => Dir::ClientCfg.loc().join("webpack.development.config.js"),
            File::WebpackProdConfig => Dir::ClientCfg.loc().join("webpack.production.config.js"),
        }
    }

    pub fn file_name(&self) -> String {
        let loc = self.loc();
        let os_filename = loc
            .file_name()
            .unwrap_or_else(|| panic!("Failed to get filename from {}", loc.display()));
        os_filename
            .to_str()
            .unwrap_or_else(|| {
                panic!(
                    "Failed to convert OsStr {:?} to &str for {}",
                    os_filename,
                    loc.display(),
                )
            })
            .to_string()
    }

    pub fn exists(&self) -> bool {
        self.loc().exists()
    }

    pub fn relative_to(&self, dir: &Dir) -> String {
        let dir_loc = dir.loc();
        let file_loc = self.loc();
        let os_path = file_loc
            .strip_prefix(&dir_loc)
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to build relative path for file {} over the directory {}: {}",
                    file_loc.display(),
                    dir_loc.display(),
                    err
                )
            })
            .to_str();
        os_path
            .unwrap_or_else(|| {
                panic!(
                    "Failed to convert OsStr {:?} to &str for file {} and directory {}",
                    os_path,
                    file_loc.display(),
                    dir_loc.display(),
                )
            })
            .to_string()
    }
}
