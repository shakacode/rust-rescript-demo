#[macro_use]
extern crate lazy_static;

use std::process;

use clap::{clap_app, App};
use tokio::io;

use cfg::Cfg;
use cmd::{Cmd, LongLivedProcess, OneOffCmd};
use instr::Instr;
use loc::Dir;
use services::*;
use status::Status;

type Result = io::Result<Status>;

lazy_static! {
    static ref CFG: Cfg = Cfg::load();
}

#[tokio::main]
async fn main() {
    let app = clap_app!(cli =>
        (version: "0.0.1")
        (about: "Rust + ReScript Demo CLI")
        (author: "Alex Fedoseev <alex.fedoseev@gmail.com>")
        (@subcommand app => (about: "Runs the app"))
        (@subcommand api =>
            (about: "API server commands")
            (@arg watch: -w --watch "Recompiles an API server on a source change")
        )
        (@subcommand database =>
            (about: "Database management")
            (visible_aliases: &["db"])
            (@subcommand create => (about: "Creates database"))
            (@subcommand drop => (about: "Drops database"))
            (@subcommand prepare => (about: "Creates/updates database JSON schema used by the app"))
            (@subcommand reset =>
              (about: "Resets Postgres datbases")
              (@arg prepare: --prepare "Creates/updates JSON schema used by the app")
            )
            (@subcommand migrations =>
                (about: "Postgres migrations")
                (visible_aliases: &["mg", "mig"])
                (@subcommand new =>
                  (about: "Creates a migration")
                  (@setting AllowExternalSubcommands)
                )
                (@subcommand run => (about: "Runs migrations"))
            )
        )
    );

    match run(app).await {
        Ok(Status::Own(Ok(()))) => {
            printer::print_done();
            process::exit(0);
        }
        Ok(Status::Piped(status)) => match status.code() {
            Some(0) => {
                printer::print_done();
                process::exit(0);
            }
            Some(code) => {
                printer::print_non_zero_exit_code(code);
                process::exit(code)
            }
            None => {
                printer::print_warning("No exit code.");
                process::exit(1)
            }
        },
        Ok(Status::Own(Err(error))) | Err(error) => {
            printer::print_error(error);
            process::exit(1);
        }
    };
}

async fn run(app: App<'_>) -> Result {
    let matches = app.clone().get_matches();

    let instr = match matches.subcommand() {
        Some(("app", _)) => {
            Instr::RunLongLivedProcessPool(vec![docker::compose::up(), api::watch()])
        }
        Some(("api", args)) => {
            if args.is_present("watch") {
                Instr::RunLongLivedProcess(api::watch())
            } else {
                Instr::RunLongLivedProcess(api::up())
            }
        }
        Some(("database", database)) => match database.subcommand() {
            Some(("create", _)) => {
                match postgres::run_one_off_cmds_against_db(vec![postgres::create_database()]).await
                {
                    Ok(cmds) => Instr::RunOneOffCmdSeq(cmds),
                    Err(err) => Instr::PrintErrorAndExit(err),
                }
            }
            Some(("drop", _)) => {
                match postgres::run_one_off_cmds_against_db(vec![postgres::drop_database()]).await {
                    Ok(cmds) => Instr::RunOneOffCmdSeq(cmds),
                    Err(err) => Instr::PrintErrorAndExit(err),
                }
            }
            Some(("prepare", _)) => {
                match postgres::run_one_off_cmds_against_db(vec![postgres::prepare_database()])
                    .await
                {
                    Ok(cmds) => Instr::RunOneOffCmdSeq(cmds),
                    Err(err) => Instr::PrintErrorAndExit(err),
                }
            }
            Some(("reset", args)) => {
                let mut cmds = vec![
                    postgres::drop_database(),
                    postgres::create_database(),
                    postgres::run_migrations(),
                ];

                if args.is_present("prepare") {
                    cmds.push(postgres::prepare_database());
                }

                match postgres::run_one_off_cmds_against_db(cmds).await {
                    Ok(cmds) => Instr::RunOneOffCmdSeq(cmds),
                    Err(err) => Instr::PrintErrorAndExit(err),
                }
            }
            Some(("migrations", migrations)) => match migrations.subcommand() {
                Some(("new", migration)) => match migration.subcommand() {
                    Some((migration, _)) => {
                        Instr::RunOneOffCmd(postgres::create_migration(migration.to_string()))
                    }
                    None => Instr::PrintHelpWithErrorAndExit {
                        err: error::new("You must provide a migration name"),
                        cmd: &["database", "migrations", "new"],
                    },
                },
                Some(("run", _)) => {
                    match postgres::run_one_off_cmds_against_db(vec![postgres::run_migrations()])
                        .await
                    {
                        Ok(cmds) => Instr::RunOneOffCmdSeq(cmds),
                        Err(err) => Instr::PrintErrorAndExit(err),
                    }
                }
                Some(_) | None => Instr::PrintHelpAndExit {
                    cmd: &["database", "migrations"],
                },
            },
            Some(_) | None => Instr::PrintHelpAndExit { cmd: &["database"] },
        },
        Some(_) | None => Instr::PrintHelpAndExit { cmd: &[] },
    };

    instr::exec(instr, app).await
}

mod status {
    use std::{io, process::ExitStatus, result};

    pub enum Status {
        Own(result::Result<(), io::Error>),
        Piped(ExitStatus),
    }

    impl Status {
        pub fn ok() -> Self {
            Self::Own(Ok(()))
        }

        pub fn err(error: io::Error) -> Self {
            Self::Own(Err(error))
        }

        pub fn piped(status: ExitStatus) -> Self {
            Self::Piped(status)
        }

        pub fn success(&self) -> bool {
            match self {
                Self::Own(Ok(())) => true,
                Self::Own(Err(_)) => false,
                Self::Piped(status) => status.success(),
            }
        }
    }
}

mod error {
    use std::io::{Error, ErrorKind};

    pub fn new(msg: &str) -> Error {
        Error::new(ErrorKind::Other, msg)
    }
}

mod loc {
    use std::path::PathBuf;

    lazy_static! {
        static ref ROOT: Root = Root::new();
    }

    struct Root(PathBuf);

    impl Root {
        pub fn new() -> Self {
            let cwd =
                std::env::current_dir().expect("Failed to get current directory of the process");
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
    }

    impl Dir {
        pub fn loc(&self) -> PathBuf {
            match self {
                Dir::Root => ROOT.path(),
                Dir::Api => Dir::Root.loc().join("api"),
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
    }
}

mod instr {
    use std::{
        io,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        time::{Duration, Instant},
    };

    use clap::App;
    use tokio::{signal, task, time};

    use crate::{colors, error, printer, LongLivedProcess, OneOffCmd, Result, Status};

    pub enum Instr {
        RunOneOffCmd(OneOffCmd),
        RunOneOffCmdSeq(Vec<OneOffCmd>),
        RunLongLivedProcess(LongLivedProcess),
        RunLongLivedProcessPool(Vec<LongLivedProcess>),
        PrintHelpAndExit {
            cmd: &'static [&'static str],
        },
        PrintErrorAndExit(io::Error),
        PrintHelpWithErrorAndExit {
            err: io::Error,
            cmd: &'static [&'static str],
        },
    }

    pub async fn exec(instr: Instr, app: App<'_>) -> Result {
        match instr {
            Instr::RunOneOffCmd(cmd) => cmd.exec().await,
            Instr::RunOneOffCmdSeq(cmds) => {
                let mut iter = cmds.iter();
                while let Some(cmd) = iter.next() {
                    let status = cmd.exec().await?;
                    if !status.success() {
                        return Ok(status);
                    }
                }
                Ok(Status::ok())
            }
            Instr::RunLongLivedProcess(cmd) => cmd.exec(colors::one()).await,
            Instr::RunLongLivedProcessPool(pool) => {
                let pool_size = pool.len();
                let exited_processes = Arc::new(AtomicUsize::new(0));

                let mut colors = colors::many(pool_size as u8);

                for process in pool {
                    let color = colors.pop().unwrap();
                    let exited_processes = exited_processes.clone();

                    task::spawn(async move {
                        let tag = console::style(process.tag()).fg(color).bold();

                        match process.exec(color).await {
                            Ok(Status::Own(Ok(()))) => printer::print_info(&format!(
                                "Process {} successfully exited.",
                                tag,
                            )),
                            Ok(Status::Piped(status)) => match status.code() {
                                Some(0) => printer::print_info(&format!(
                                    "Process {} exited with code 0.",
                                    tag,
                                )),
                                Some(code) => printer::print_warning(&format!(
                                    "Process {} exited with non-zero code: {}",
                                    tag, code
                                )),
                                None => printer::print_info(&format!(
                                    "Process {} exited without code.",
                                    tag,
                                )),
                            },
                            Ok(Status::Own(Err(error))) | Err(error) => {
                                printer::print_error(error::new(&format!(
                                    "Process {} exited with error: {}",
                                    tag, error
                                )))
                            }
                        }
                        exited_processes.fetch_add(1, Ordering::Relaxed);
                    });
                }

                signal::ctrl_c().await.unwrap();

                let timeout = Duration::from_secs(10);
                let expire = Instant::now() + timeout;
                while exited_processes.load(Ordering::Relaxed) < pool_size {
                    if Instant::now() > expire {
                        printer::print_warning("Timeout. Exiting.");
                        // TODO: Send SIGKILL to all processes on Unix (on Windows - ?)
                        break;
                    }
                    time::sleep(Duration::from_millis(500)).await;
                }

                Ok(Status::ok())
            }
            Instr::PrintHelpAndExit { cmd } => {
                printer::print_help(&app, cmd);
                return Ok(Status::ok());
            }
            Instr::PrintErrorAndExit(err) => {
                return Ok(Status::err(err));
            }
            Instr::PrintHelpWithErrorAndExit { err, cmd } => {
                printer::print_help(&app, cmd);
                return Ok(Status::err(err));
            }
        }
    }
}

mod env {
    use std::collections::HashMap;

    pub type Vars = HashMap<String, String>;

    pub fn empty() -> Vars {
        HashMap::new()
    }

    pub fn one<K: ToString, V: ToString>(k: K, v: V) -> Vars {
        let mut vars = HashMap::with_capacity(1);
        vars.insert(k.to_string(), v.to_string());
        vars
    }
}

mod cfg {
    use std::{collections::HashMap, iter::FromIterator};

    use crate::env::Vars;

    pub struct Cfg(Vars);

    impl Cfg {
        pub fn load() -> Self {
            #[allow(deprecated)] // it was undeprecated
            let vars = dotenv::from_path_iter(".env").unwrap().map(Result::unwrap);
            Self(HashMap::from_iter(vars))
        }

        pub fn get(&self) -> Vars {
            self.0.to_owned()
        }

        pub fn pg_host(&self) -> String {
            self.0
                .get("PG_HOST")
                .expect("Failed to get PG_HOST")
                .to_string()
        }

        pub fn pg_port(&self) -> String {
            self.0
                .get("PG_PORT")
                .expect("Failed to get PG_PORT")
                .to_string()
        }

        pub fn pg_user(&self) -> String {
            self.0
                .get("PG_USER")
                .expect("Failed to get PG_USER")
                .to_string()
        }

        pub fn pg_password(&self) -> String {
            self.0
                .get("PG_PASSWORD")
                .expect("Failed to get PG_PASSWORD")
                .to_string()
        }

        pub fn pg_database(&self) -> String {
            self.0
                .get("PG_DATABASE")
                .expect("Failed to get PG_DB_NAME")
                .to_string()
        }

        pub fn pg_url(&self) -> String {
            format!(
                "postgres://{user}:{password}@{host}:{port}/{database}",
                user = self.pg_user(),
                password = self.pg_password(),
                host = self.pg_host(),
                port = self.pg_port(),
                database = self.pg_database(),
            )
        }
    }
}

mod cmd {
    use std::process::Stdio;

    use console::Color;
    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        process::Command,
        task,
    };

    use crate::{env, Dir, Result, Status};

    #[derive(Clone)]
    pub struct Cmd {
        pub run: String,
        pub env: env::Vars,
        pub dir: Dir,
        pub msg: &'static str,
    }

    impl Cmd {
        pub fn shell() -> &'static str {
            if cfg!(unix) {
                "/bin/sh"
            } else if cfg!(windows) {
                "cmd"
            } else {
                panic!("Unsupported operating system")
            }
        }

        pub fn shelled(cmd: &str) -> Vec<&str> {
            if cfg!(unix) {
                vec!["-c", &cmd]
            } else if cfg!(windows) {
                vec!["/c", &cmd]
            } else {
                panic!("Unsupported operating system")
            }
        }
    }

    pub struct OneOffCmd(Cmd);

    impl OneOffCmd {
        pub fn new(cmd: Cmd) -> Self {
            Self(cmd)
        }

        pub async fn exec(&self) -> Result {
            let cmd = &self.0;

            println!(
                "❯ {} {}",
                console::style(format!("{}:", cmd.msg)).bold(),
                console::style(format!("$ {} @ {}", cmd.run, cmd.dir.display())).dim()
            );

            Command::new(Cmd::shell())
                .args(Cmd::shelled(&cmd.run))
                .envs(cmd.env.to_owned())
                .current_dir(cmd.dir.loc())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect(&format!("Failed to spawn the process: {}", cmd.run))
                .wait_with_output()
                .await
                .map(|res| Status::piped(res.status))
        }
    }

    pub struct LongLivedProcess {
        tag: &'static str,
        cmd: Cmd,
    }

    impl LongLivedProcess {
        pub fn new(tag: &'static str, cmd: Cmd) -> Self {
            Self { tag, cmd }
        }

        pub fn tag(&self) -> &str {
            self.tag
        }

        pub async fn exec(&self, color: Color) -> Result {
            let cmd = &self.cmd;
            let tag = console::style(format!("{}  |", self.tag)).fg(color).bold();

            println!(
                "❯ {} {}",
                console::style(format!("{}...", cmd.msg)).bold(),
                console::style(format!("$ {} @ {}", cmd.run, cmd.dir.display())).dim()
            );

            let mut child = Command::new(Cmd::shell())
                .args(Cmd::shelled(&cmd.run))
                .envs(cmd.env.to_owned())
                .current_dir(cmd.dir.loc())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect(&format!("Failed to spawn the process: {}", cmd.run));

            let child_stdout = child
                .stdout
                .take()
                .expect(&format!("Failed to get a handle to stdout of {}", cmd.run));

            let child_stderr = child
                .stderr
                .take()
                .expect(&format!("Failed to get a handle to stderr of {}", cmd.run));

            let mut child_stdout_reader = BufReader::new(child_stdout).lines();
            let mut child_stderr_reader = BufReader::new(child_stderr).lines();

            task::spawn({
                let tag = tag.clone();
                async move {
                    while let Some(line) = child_stdout_reader.next_line().await.unwrap() {
                        println!("{} {}", tag, line);
                    }
                }
            });

            task::spawn({
                let tag = tag.clone();
                async move {
                    while let Some(line) = child_stderr_reader.next_line().await.unwrap() {
                        eprintln!("{} {}", tag, line);
                    }
                }
            });

            child
                .wait_with_output()
                .await
                .map(|res| Status::piped(res.status))
        }
    }
}

mod printer {
    use std::io;

    use clap::App;
    use console::Color;

    pub fn print_info(msg: &str) {
        let badge = console::style(" INFO ")
            .fg(Color::Color256(255))
            .bg(Color::Color256(39));

        println!("\n{} {}", badge, msg);
    }

    pub fn print_warning(msg: &str) {
        let badge = console::style(" WARNING ")
            .fg(Color::Color256(94))
            .bg(Color::Yellow);

        eprintln!("\n{} {}", badge, msg);
    }

    pub fn print_error(error: io::Error) {
        let badge = console::style(" ERROR ")
            .fg(Color::Color256(255))
            .bg(Color::Red);
        let msg = console::style(error).red().bold();

        eprintln!("\n{} {}", badge, msg);
    }

    pub fn print_non_zero_exit_code(code: i32) {
        let badge = console::style(" ERROR ")
            .fg(Color::Color256(255))
            .bg(Color::Red);
        let msg = console::style(format!("Exit code: {}", code)).red().bold();

        eprintln!("\n{} {}", badge, msg);
    }

    pub fn print_done() {
        println!("\n✨ Done.");
    }

    pub fn print_help(app: &App, cmds: &[&str]) -> () {
        match cmds {
            &[] => app.to_owned().print_help().unwrap(),
            &[cmd] => app
                .find_subcommand(cmd)
                .unwrap()
                .to_owned()
                .print_help()
                .unwrap(),
            &[cmd, ..] => {
                let app = app.find_subcommand(cmd).unwrap();
                let cmds = &cmds[1..];
                print_help(app, cmds)
            }
        }
    }
}

mod colors {
    use console::Color;

    pub fn one() -> Color {
        many(1).pop().unwrap()
    }

    pub fn many(n: u8) -> Vec<Color> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut colors = vec![
            // Color::Red, // Red is for errors
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
        ];
        if n <= colors.len() as u8 {
            colors.shuffle(&mut thread_rng());
            colors.truncate(n as usize);
            colors
        } else {
            // TODO: Handle more than 5 proceses
            unimplemented!()
        }
    }
}

mod services {
    pub mod api {
        use crate::{Cmd, Dir, LongLivedProcess, CFG};

        pub fn up() -> LongLivedProcess {
            LongLivedProcess::new(
                "api",
                Cmd {
                    run: "cargo run --package=api --color=always".to_string(),
                    env: CFG.get(),
                    dir: Dir::Root,
                    msg: "Running API server",
                },
            )
        }

        pub fn watch() -> LongLivedProcess {
            LongLivedProcess::new(
                "api",
                Cmd {
                    run: "cargo watch -x 'run --package=api --color=always'".to_string(),
                    env: CFG.get(),
                    dir: Dir::Root,
                    msg: "Running reloadable API server",
                },
            )
        }
    }

    pub mod postgres {
        use std::io;

        use crate::{env, services::docker, Cmd, Dir, OneOffCmd, CFG};

        pub fn create_database() -> OneOffCmd {
            OneOffCmd::new(Cmd {
                run: "sqlx database create".to_string(),
                env: env::one("DATABASE_URL", CFG.pg_url()),
                dir: Dir::Root,
                msg: "Creating database",
            })
        }

        pub fn drop_database() -> OneOffCmd {
            OneOffCmd::new(Cmd {
                run: "sqlx database drop -y".to_string(),
                env: env::one("DATABASE_URL", CFG.pg_url()),
                dir: Dir::Root,
                msg: "Dropping database",
            })
        }

        pub fn prepare_database() -> OneOffCmd {
            OneOffCmd::new(Cmd {
                run: "cargo sqlx prepare".to_string(),
                env: env::one("DATABASE_URL", CFG.pg_url()),
                dir: Dir::Api,
                msg: "Preparing database",
            })
        }

        pub fn create_migration(name: String) -> OneOffCmd {
            OneOffCmd::new(Cmd {
                run: format!("sqlx migrate add {}", name),
                env: env::one("DATABASE_URL", CFG.pg_url()),
                dir: Dir::Api,
                msg: "Creating migration",
            })
        }

        pub fn run_migrations() -> OneOffCmd {
            OneOffCmd::new(Cmd {
                run: "sqlx migrate run".to_string(),
                env: env::one("DATABASE_URL", CFG.pg_url()),
                dir: Dir::Api,
                msg: "Running migrations",
            })
        }

        pub async fn run_one_off_cmds_against_db(
            cmds: Vec<OneOffCmd>,
        ) -> Result<Vec<OneOffCmd>, io::Error> {
            docker::compose::pg_status().await.map(|pg_status| {
                let mut acc = vec![];

                if let docker::compose::ServiceStatus::Stopped = pg_status {
                    acc.push(docker::compose::start_detached_pg());
                }

                acc.extend(cmds);

                if let docker::compose::ServiceStatus::Stopped = pg_status {
                    acc.push(docker::compose::stop_pg());
                }

                acc
            })
        }
    }

    pub mod docker {
        pub mod compose {
            use std::{io, process::Stdio, str};

            use tokio::process::Command;

            use crate::{env, Cmd, Dir, LongLivedProcess, OneOffCmd};

            const PG_SERVICE_ID: &str = "pg";

            pub enum ServiceStatus {
                Running,
                Stopped,
            }

            pub fn up() -> LongLivedProcess {
                LongLivedProcess::new(
                    "docker",
                    Cmd {
                        run: "docker-compose up".to_string(),
                        env: env::empty(),
                        dir: Dir::Root,
                        msg: "Running Docker services",
                    },
                )
            }

            pub fn start_detached_pg() -> OneOffCmd {
                OneOffCmd::new(Cmd {
                    run: format!("docker-compose up -d {}", PG_SERVICE_ID),
                    env: env::empty(),
                    dir: Dir::Root,
                    msg: "Running detached Postgres service",
                })
            }

            pub fn stop_pg() -> OneOffCmd {
                OneOffCmd::new(Cmd {
                    run: format!("docker-compose stop {}", PG_SERVICE_ID),
                    env: env::empty(),
                    dir: Dir::Root,
                    msg: "Stopping Postgres service",
                })
            }

            pub async fn pg_status() -> Result<ServiceStatus, io::Error> {
                let cmd = "docker-compose ps --services --filter status=running";

                Command::new(Cmd::shell())
                    .args(Cmd::shelled(cmd))
                    .current_dir(Dir::Root.loc())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect(&format!(
                        "Failed to spawn the process that checks if `{}` service is running",
                        PG_SERVICE_ID
                    ))
                    .wait_with_output()
                    .await
                    .and_then(|output| match output.status.code() {
                        Some(0) | None => match str::from_utf8(output.stdout.as_slice()) {
                            Ok(services) => {
                                let mut services = services.lines();
                                while let Some(service) = services.next() {
                                    if service == PG_SERVICE_ID {
                                        return Ok(ServiceStatus::Running);
                                    }
                                }
                                Ok(ServiceStatus::Stopped)
                            }
                            Err(error) => Err(io::Error::new(
                                io::ErrorKind::Other,
                                format!("Failed to parse output from {}: {}", cmd, error),
                            )),
                        },
                        Some(_) => Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("Command {} exited with non-zero code", cmd),
                        )),
                    })
            }
        }
    }
}
