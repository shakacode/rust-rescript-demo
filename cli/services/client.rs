use crate::{env, Dir, EnvData};

fn env() -> EnvData {
    // PATH with `node_modules/.bin` included
    EnvData::one(
        "PATH",
        env::path::extend(Dir::Client.extend(&["node_modules", ".bin"])),
    )
}

pub mod rescript {
    use crate::{Cmd, Dir, EnvData, Process};

    #[derive(Clone)]
    pub enum LogLevel {
        Trace,
        Debug,
        Info,
        Warn,
        Error,
    }

    impl LogLevel {
        pub fn to_str(&self) -> &'static str {
            match self {
                LogLevel::Trace => "trace",
                LogLevel::Debug => "debug",
                LogLevel::Info => "info",
                LogLevel::Warn => "warn",
                LogLevel::Error => "error",
            }
        }
    }

    impl From<&str> for LogLevel {
        fn from(str: &str) -> LogLevel {
            match str {
                "trace" => LogLevel::Trace,
                "debug" => LogLevel::Debug,
                "info" => LogLevel::Info,
                "warn" | "warning" => LogLevel::Warn,
                "error" => LogLevel::Error,
                _ => panic!("Unexpected value of ReScript log level"),
            }
        }
    }

    fn env(log_level: Option<LogLevel>) -> EnvData {
        let rescript_env = {
            let mut env = vec![("NINJA_ANSI_FORCED", "1")];
            if let Some(log_level) = log_level {
                env.push(("RES_LOG", log_level.to_str()))
            }
            env
        };
        super::env().merge(EnvData::from_vec(rescript_env))
    }

    pub fn make_world(log_level: Option<LogLevel>, clean: bool) -> Cmd {
        Cmd {
            run: if clean {
                "bsb -clean-world -make-world".to_string()
            } else {
                "bsb -make-world".to_string()
            },
            env: self::env(log_level),
            dir: Dir::Client,
            msg: "Building ReScript app",
        }
    }

    pub fn watch(log_level: Option<LogLevel>) -> Process {
        Process::new(
            "rescript",
            Cmd {
                run: "bsb -w".to_string(),
                env: self::env(log_level),
                dir: Dir::Client,
                msg: "Watching ReScript app",
            },
        )
    }

    pub fn make_and_watch_world(log_level: Option<LogLevel>, clean: bool) -> Process {
        Process::new(
            "rescript",
            Cmd {
                run: if clean {
                    "bsb -clean-world -make-world -w".to_string()
                } else {
                    "bsb -make-world -w".to_string()
                },
                env: self::env(log_level),
                dir: Dir::Client,
                msg: "Building and watching ReScript app",
            },
        )
    }

    pub fn clean_world() -> Cmd {
        Cmd {
            run: "bsb -clean-world".to_string(),
            env: self::env(None),
            dir: Dir::Client,
            msg: "Cleaning ReScript app",
        }
    }
}

pub mod webpack {
    use crate::{Cmd, Dir, Env, File, Process, CFG};

    // TODO: Extract production build into own function with own server
    pub fn serve(env: &Env) -> Process {
        let client_dir = Dir::Client;

        Process::new(
            "webpack",
            Cmd {
                run: format!(
                    "webpack serve --host {host} --port {port} --config {config}",
                    host = CFG.web_host(env),
                    port = CFG.web_port(env),
                    config = match env {
                        Env::Dev => File::WebpackDevConfig.relative_to(&client_dir),
                        Env::Prod => File::WebpackProdConfig.relative_to(&client_dir),
                        Env::Test => unimplemented!(),
                    }
                ),
                env: super::env()
                    .merge(CFG.env(env))
                    .add("NODE_ENV", env.to_str()),
                dir: client_dir,
                msg: match env {
                    Env::Dev => "Running Webpack development server",
                    Env::Prod => "Running Webpack production server",
                    Env::Test => "Running Webpack test server",
                },
            },
        )
    }
}

pub mod graphql {
    use crate::{Cmd, Dir, Env, CFG};

    pub fn write_schema(env: &Env) -> Cmd {
        Cmd {
            run: format!(
                "get-graphql-schema {} --json > graphql_schema.json",
                CFG.api_graphql_url(env).format()
            ),
            env: super::env(),
            dir: Dir::Client,
            msg: match env {
                Env::Dev => "Writing GraphQL schema against development server",
                Env::Prod => "Writing GraphQL schema against production server",
                Env::Test => "Writing GraphQL schema against test server",
            },
        }
    }
}
