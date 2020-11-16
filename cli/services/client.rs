use crate::{env, Dir, Env};

fn env() -> Env {
    // PATH with `node_modules/.bin` included
    env::one(
        "PATH",
        env::path::extend(Dir::Client.extend(&["node_modules", ".bin"])),
    )
}

pub mod rescript {
    use crate::{env, Cmd, Dir, Env, Process};

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

    fn env(log_level: Option<LogLevel>) -> Env {
        let rescript_env = {
            let mut env = vec![("NINJA_ANSI_FORCED", "1")];
            if let Some(log_level) = log_level {
                env.push(("RES_LOG", log_level.to_str()))
            }
            env
        };
        env::merge(super::env(), env::new(rescript_env))
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
    use crate::{env, Cmd, Dir, Process, CFG};

    pub fn serve() -> Process {
        Process::new(
            "webpack",
            Cmd {
                run: format!(
                    "webpack serve --host {} --port {}",
                    host = CFG.web_host(),
                    port = CFG.web_port()
                ),
                env: env::merge(CFG.env(), super::env()),
                dir: Dir::Client,
                msg: "Running Webpack development server",
            },
        )
    }
}

pub mod graphql {
    use crate::{Cmd, Dir, CFG};

    pub fn write_schema() -> Cmd {
        Cmd {
            run: format!(
                "get-graphql-schema {} --json > graphql_schema.json",
                CFG.api_graphql_url().format()
            ),
            env: super::env(),
            dir: Dir::Client,
            msg: "Writing GraphQL schema",
        }
    }
}
