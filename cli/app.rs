use clap::clap_app;

use crate::{error, services::*, Env, Error, Exec, Result, TcpAddr, CFG};

pub struct App(clap::App<'static>);

impl App {
    pub fn new() -> App {
        App(clap_app!(rrd =>
            (version: "0.0.1")
            (about: "Rust + ReScript Demo CLI")
            (author: "Alex Fedoseev <alex.fedoseev@gmail.com>")
            (@setting ArgRequiredElseHelp)
            (@subcommand setup => (about: "Sets up environment incl. database, dependencies, etc."))
            (@subcommand update => (about: "Updates environment incl. database, dependencies, etc."))
            (@subcommand reset => (about: "Resets environment incl. database, dependencies, etc."))
            (@subcommand develop =>
              (visible_aliases: &["dev"])
              (about: "Runs the app incl. api server, web client etc")
              (@arg prod: -p --production "Runs a production build")
              (@arg "rescript-log-level": --"rescript-log-level" +takes_value "Sets log level for ReScript app")
            )
            (@subcommand api =>
                (about: "API server commands")
                (@setting ArgRequiredElseHelp)
                (@subcommand build =>
                  (about: "Builds API")
                  (@arg release: -r --release "Builds release")
                )
                (@subcommand clean => (about: "Cleans API artefacts"))
                (@subcommand run =>
                    (about: "Runs API server")
                    (@group env =>
                      (@arg release: -r --release "Runs release build")
                    )
                    (@arg watch: -w --watch "Recompiles an API server on a source change")
                )
            )
            (@subcommand rescript =>
                (about: "ReScript commands")
                (visible_aliases: &["res"])
                (@setting ArgRequiredElseHelp)
                (@subcommand build =>
                  (about: "Builds ReScript app")
                  (visible_aliases: &["b"])
                  (@arg clean: -c --clean "Cleans ReScript app")
                  (@arg watch: -w --watch "Watches ReScript app")
                  (@arg log: -l --log +takes_value "Sets log level for ReScript app")
                )
                (@subcommand watch =>
                  (about: "Watches ReScript app")
                  (visible_aliases: &["w"])
                  (@arg log: -l --log +takes_value "Sets log level for ReScript app")
                )
                (@subcommand clean =>
                  (about: "Cleans ReScript app artefacts")
                  (visible_aliases: &["c"])
                )
                (@subcommand graphql =>
                  (about: "Writes GraphQL schema used by ReScript app")
                  (visible_aliases: &["gql"])
                  (@group env =>
                      (@arg dev: -d --development "Uses development API server [default]")
                      (@arg prod: -p --production "Uses production API server")
                      (@arg test: -t --test "Uses test API server")
                  )
                )
            )
            (@subcommand db =>
                (about: "Database commands")
                (@setting ArgRequiredElseHelp)
                (@subcommand create =>
                  (about: "Creates database")
                  (@group env =>
                      (@attributes ... +required)
                      (@arg dev: -d --development "Creates development database")
                      (@arg prod: -p --production "Creates production database")
                      (@arg test: -t --test "Creates test database")
                  )
                )
                (@subcommand drop =>
                  (about: "Drops database")
                  (@group env =>
                      (@attributes ... +required)
                      (@arg dev: -d --development "Drops development database")
                      (@arg prod: -p --production "Drops production database")
                      (@arg test: -t --test "Drops test database")
                  )
                )
                (@subcommand reset =>
                  (about: "Resets Postgres datbases")
                  (@arg prepare: --prepare "Prepares database schema")
                  (@group env =>
                      (@attributes ... +required)
                      (@arg dev: -d --development "Resets development database")
                      (@arg prod: -p --production "Resets production database")
                      (@arg test: -t --test "Resets test database")
                  )
                )
                (@subcommand schema =>
                  (about: "Prepares database schema")
                  (@group env =>
                      (@attributes ... +required)
                      (@arg dev: -d --development "Prepares schema against development database")
                      (@arg prod: -p --production "Prepares schema against production database")
                      (@arg test: -t --test "Prepares schema against test database")
                  )
                )
                (@subcommand migrations =>
                    (about: "Postgres migration commands")
                    (visible_aliases: &["mg", "mig"])
                    (@setting ArgRequiredElseHelp)
                    (@subcommand new =>
                      (about: "Creates a migration")
                      (@setting AllowExternalSubcommands)
                    )
                    (@subcommand run =>
                      (about: "Runs migrations")
                      (@group env =>
                          (@attributes ... +required)
                          (@arg dev: -d --development "Runs migrations against development database")
                          (@arg prod: -p --production "Runs migrations against production database")
                          (@arg test: -t --test "Runs migrations against test database")
                      )
                    )
                )
            )
            (@subcommand cli =>
              (about: "CLI commands")
              (@setting ArgRequiredElseHelp)
              (@subcommand install =>
                (about: "Installs this CLI")
                (visible_aliases: &["i"])
              )
              (@subcommand update =>
                (about: "Updates this CLI")
                (visible_aliases: &["u"])
              )
            )
        ))
    }

    pub async fn run(self) -> Result {
        let app = self.0;
        let matches = app.get_matches();

        match matches.subcommand() {
            Some(("setup", _)) => {
                // Checking system and environment
                sys::ensure_prerequisites().await?;

                // Setting up API
                Exec::cmd(api::build_dev()).await?;

                // Setting up Yarn
                Exec::cmd(yarn::install()).await?;

                // Setting up Postgres
                let mut cmds = vec![];
                for env in CFG.envs_with_unique_dbs() {
                    cmds.extend(vec![
                        postgres::create_database(&env),
                        postgres::run_migrations(&env),
                    ])
                }
                postgres::run_one_off_cmds_against_db(cmds).await
            }
            Some(("update", _)) => {
                // Updating CLI
                // NOTE: It's prolly better to run from post-merge/post-checkout git hook, though the latter is annoying
                Exec::cmd(cli::release(cli::ReleaseCtx::Update)).await?;

                // Checking system and environment
                sys::ensure_prerequisites().await?;

                // Updating API
                Exec::cmd(api::build_dev()).await?;

                // Updating Yarn
                Exec::cmd(yarn::install()).await?;

                // Running migrations
                let mut cmds = vec![];
                for env in CFG.envs_with_unique_dbs() {
                    cmds.extend(vec![postgres::run_migrations(&env)])
                }
                postgres::run_one_off_cmds_against_db(cmds).await
            }
            Some(("reset", _)) => {
                // Checking system and environment
                sys::ensure_prerequisites().await?;

                // Resetting API
                Exec::cmd(api::clean()).await?;
                Exec::cmd(api::build_dev()).await?;

                // Resetting Yarn
                Exec::cmd(yarn::remove_client_node_modules()).await?;
                Exec::cmd(yarn::remove_root_node_modules()).await?;
                Exec::cmd(yarn::install()).await?;

                // Resetting Postgres
                let mut cmds = vec![];
                for env in CFG.envs_with_unique_dbs() {
                    cmds.extend(vec![
                        postgres::drop_database(&env),
                        postgres::create_database(&env),
                        postgres::run_migrations(&env),
                    ])
                }
                postgres::run_one_off_cmds_against_db(cmds).await
            }
            Some(("develop", args)) => {
                let rescript_log_level = match args.value_of("rescript-log-level") {
                    None => client::rescript::LogLevel::Debug,
                    Some(val) => val.into(),
                };
                Exec::cmd(client::rescript::make_world(
                    Some(rescript_log_level.to_owned()),
                    true,
                ))
                .await?;

                if args.is_present("prod") {
                    Exec::process_pool(vec![
                        docker::compose::up(),
                        api::watch_release(),
                        client::rescript::watch(Some(rescript_log_level)),
                        client::webpack::serve(&Env::Prod),
                    ])
                    .await
                } else {
                    Exec::process_pool(vec![
                        docker::compose::up(),
                        api::watch_dev(),
                        client::rescript::watch(Some(rescript_log_level)),
                        client::webpack::serve(&Env::Dev),
                    ])
                    .await
                }
            }
            Some(("api", api)) => match api.subcommand() {
                Some(("build", args)) => {
                    if args.is_present("release") {
                        Exec::cmd(api::build_release()).await
                    } else {
                        Exec::cmd(api::build_dev()).await
                    }
                }
                Some(("clean", _)) => Exec::cmd(api::clean()).await,
                Some(("run", args)) => {
                    if args.is_present("release") {
                        if args.is_present("watch") {
                            Exec::process(api::watch_release()).await
                        } else {
                            Exec::process(api::run_release()).await
                        }
                    } else {
                        if args.is_present("watch") {
                            Exec::process(api::watch_dev()).await
                        } else {
                            Exec::process(api::run_dev()).await
                        }
                    }
                }
                Some(_) | None => Err(Error::NothingToExecute),
            },
            Some(("rescript", rescript)) => match rescript.subcommand() {
                Some(("build", args)) => {
                    let log_level = args.value_of("log").map(Into::into);
                    let clean = args.is_present("clean");
                    if args.is_present("watch") {
                        Exec::process(client::rescript::make_and_watch_world(log_level, clean))
                            .await
                    } else {
                        Exec::cmd(client::rescript::make_world(log_level, clean)).await
                    }
                }
                Some(("watch", args)) => {
                    let log_level = args.value_of("log").map(Into::into);
                    Exec::process(client::rescript::watch(log_level)).await
                }
                Some(("clean", _)) => Exec::cmd(client::rescript::clean_world()).await,
                Some(("graphql", args)) => {
                    let env = App::env_from_args(args).unwrap_or(Env::Dev);
                    match CFG.api_health_url(&env).ping().await {
                        Ok(()) => Exec::cmd(client::graphql::write_schema(&env)).await,
                        Err(()) => {
                            Exec::dependent_cmd(
                                client::graphql::write_schema(&env),
                                match env {
                                    Env::Dev => api::run_dev(),
                                    Env::Prod => api::run_release(),
                                    Env::Test => unimplemented!(),
                                },
                                TcpAddr {
                                    host: CFG.api_host(&env),
                                    port: CFG.api_port(&env),
                                },
                            )
                            .await
                        }
                    }
                }
                Some(_) | None => Err(Error::NothingToExecute),
            },
            Some(("db", db)) => match db.subcommand() {
                Some(("create", args)) => {
                    let mut cmds = vec![];
                    for env in App::envs_from_args(args) {
                        cmds.extend(vec![postgres::create_database(&env)])
                    }
                    postgres::run_one_off_cmds_against_db(cmds).await
                }
                Some(("drop", args)) => {
                    let mut cmds = vec![];
                    for env in App::envs_from_args(args) {
                        cmds.extend(vec![postgres::drop_database(&env)])
                    }
                    postgres::run_one_off_cmds_against_db(cmds).await
                }
                Some(("schema", args)) => {
                    let mut cmds = vec![];
                    for env in App::envs_from_args(args) {
                        cmds.extend(vec![postgres::prepare_database_schema(&env)])
                    }
                    postgres::run_one_off_cmds_against_db(cmds).await
                }
                Some(("reset", args)) => {
                    let envs = App::envs_from_args(args);
                    let default_env = envs.first().unwrap().clone();
                    let mut cmds = vec![];
                    for env in envs {
                        cmds.extend(vec![
                            postgres::drop_database(&env),
                            postgres::create_database(&env),
                            postgres::run_migrations(&env),
                        ])
                    }

                    if args.is_present("prepare") {
                        cmds.push(postgres::prepare_database_schema(&default_env));
                    }

                    postgres::run_one_off_cmds_against_db(cmds).await
                }
                Some(("migrations", migrations)) => match migrations.subcommand() {
                    Some(("new", migration)) => match migration.subcommand() {
                        Some((migration, _)) => {
                            Exec::cmd(postgres::create_migration(migration.to_string())).await
                        }
                        None => Err(error::other("You must provide a migration name").into()),
                    },
                    Some(("run", args)) => {
                        let mut cmds = vec![];
                        for env in App::envs_from_args(args) {
                            cmds.extend(vec![postgres::run_migrations(&env)])
                        }
                        postgres::run_one_off_cmds_against_db(cmds).await
                    }
                    Some(_) | None => Err(Error::NothingToExecute),
                },
                Some(_) | None => Err(Error::NothingToExecute),
            },
            Some(("cli", cli)) => match cli.subcommand() {
                Some(("install", _)) => Exec::cmd(cli::release(cli::ReleaseCtx::Install)).await,
                Some(("update", _)) => Exec::cmd(cli::release(cli::ReleaseCtx::Update)).await,
                Some(_) | None => Err(Error::NothingToExecute),
            },
            Some(_) | None => Err(Error::NothingToExecute),
        }
    }

    fn env_from_args(args: &clap::ArgMatches) -> Option<Env> {
        if args.is_present("dev") {
            Some(Env::Dev)
        } else if args.is_present("prod") {
            Some(Env::Prod)
        } else if args.is_present("test") {
            Some(Env::Test)
        } else {
            None
        }
    }

    fn envs_from_args(args: &clap::ArgMatches) -> Vec<Env> {
        let mut envs = vec![];
        if args.is_present("dev") {
            envs.push(Env::Dev)
        }
        if args.is_present("prod") {
            envs.push(Env::Prod)
        }
        if args.is_present("test") {
            envs.push(Env::Test)
        }
        envs
    }
}
