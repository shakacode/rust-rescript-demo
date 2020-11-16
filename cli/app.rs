use clap::clap_app;

use crate::{error, printer, services::*, Error, Exec, Result, TcpAddr, CFG};

pub struct App(clap::App<'static>);

impl App {
    pub fn new() -> App {
        App(clap_app!(rrd =>
            (version: "0.0.1")
            (about: "Rust + ReScript Demo CLI")
            (author: "Alex Fedoseev <alex.fedoseev@gmail.com>")
            (@subcommand app =>
              (about: "Runs the app")
              (@arg "rescript-log-level": --"rescript-log-level" +takes_value "Sets log level for ReScript app")
            )
            (@subcommand api =>
                (about: "API server commands")
                (@arg watch: -w --watch "Recompiles an API server on a source change")
            )
            (@subcommand rescript =>
                (about: "ReScript commands")
                (visible_aliases: &["res"])
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
                  (about: "Cleans ReScript app")
                  (visible_aliases: &["c"])
                )
                (@subcommand graphql =>
                  (about: "Writes GraphQL schema used by ReScript app")
                  (visible_aliases: &["gql"])
                )
            )
            (@subcommand database =>
                (about: "Database commands")
                (visible_aliases: &["db"])
                (@subcommand create => (about: "Creates database"))
                (@subcommand drop => (about: "Drops database"))
                (@subcommand prepare => (about: "Creates/updates database JSON schema used by the app"))
                (@subcommand reset =>
                  (about: "Resets Postgres datbases")
                  (@arg prepare: --prepare "Creates/updates JSON schema used by the app")
                )
                (@subcommand migrations =>
                    (about: "Postgres migration commands")
                    (visible_aliases: &["mg", "mig"])
                    (@subcommand new =>
                      (about: "Creates a migration")
                      (@setting AllowExternalSubcommands)
                    )
                    (@subcommand run => (about: "Runs migrations"))
                )
            )
            (@subcommand install => (about: "Installs this cli"))
            (@subcommand rebuild =>
              (about: "Rebuild this cli")
              (visible_aliases: &["reb"])
            )
        ))
    }

    pub fn inner(&self) -> &clap::App<'static> {
        &self.0
    }

    pub async fn run(&self) -> Result {
        let app = &self.0;
        let matches = app.clone().get_matches();

        match matches.subcommand() {
            Some(("app", args)) => {
                let rescript_log_level = match args.value_of("rescript-log-level") {
                    None => client::rescript::LogLevel::Debug,
                    Some(val) => val.into(),
                };

                Exec::cmd(client::rescript::make_world(
                    Some(rescript_log_level.to_owned()),
                    true,
                ))
                .await?;
                Exec::pool(vec![
                    docker::compose::up(),
                    api::watch(),
                    client::rescript::watch(Some(rescript_log_level)),
                    client::webpack::serve(),
                ])
                .await
            }
            Some(("api", args)) => {
                if args.is_present("watch") {
                    Exec::process(api::watch()).await
                } else {
                    Exec::process(api::up()).await
                }
            }
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
                Some(("graphql", _)) => match CFG.api_health_url().ping().await {
                    Ok(()) => Exec::cmd(client::graphql::write_schema()).await,
                    Err(()) => {
                        Exec::dependent_cmd(
                            client::graphql::write_schema(),
                            api::up(),
                            TcpAddr {
                                host: CFG.api_host(),
                                port: CFG.api_port(),
                            },
                        )
                        .await
                    }
                },
                Some(_) | None => Err(Error::NothingToExecute { cmd: &["rescript"] }),
            },
            Some(("database", database)) => match database.subcommand() {
                Some(("create", _)) => {
                    postgres::run_one_off_cmds_against_db(vec![postgres::create_database()]).await
                }
                Some(("drop", _)) => {
                    postgres::run_one_off_cmds_against_db(vec![postgres::drop_database()]).await
                }
                Some(("prepare", _)) => {
                    postgres::run_one_off_cmds_against_db(vec![postgres::prepare_database()]).await
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

                    postgres::run_one_off_cmds_against_db(cmds).await
                }
                Some(("migrations", migrations)) => match migrations.subcommand() {
                    Some(("new", migration)) => match migration.subcommand() {
                        Some((migration, _)) => {
                            Exec::cmd(postgres::create_migration(migration.to_string())).await
                        }
                        None => {
                            printer::print_help(app, &["database", "migrations", "new"]);
                            Err(error::other("You must provide a migration name").into())
                        }
                    },
                    Some(("run", _)) => {
                        postgres::run_one_off_cmds_against_db(vec![postgres::run_migrations()])
                            .await
                    }
                    Some(_) | None => Err(Error::NothingToExecute {
                        cmd: &["database", "migrations"],
                    }),
                },
                Some(_) | None => Err(Error::NothingToExecute { cmd: &["database"] }),
            },
            Some(("install", _)) => Exec::cmd(cli::release(cli::ReleaseCtx::Install)).await,
            Some(("rebuild", _)) => Exec::cmd(cli::release(cli::ReleaseCtx::Update)).await,
            Some(_) | None => Err(Error::NothingToExecute { cmd: &[] }),
        }
    }
}
