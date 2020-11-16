pub mod compose {
    use std::{io, process::Stdio, str};

    use tokio::process::Command;

    use crate::{env, Cmd, Dir, Process};

    const PG_SERVICE_ID: &str = "pg";

    pub enum ServiceStatus {
        Running,
        Stopped,
    }

    pub fn up() -> Process {
        Process::new(
            "docker",
            Cmd {
                run: "docker-compose up".to_string(),
                env: env::empty(),
                dir: Dir::Root,
                msg: "Running Docker services",
            },
        )
    }

    pub fn start_detached_pg() -> Cmd {
        Cmd {
            run: format!("docker-compose up -d {}", PG_SERVICE_ID),
            env: env::empty(),
            dir: Dir::Root,
            msg: "Running detached Postgres service",
        }
    }

    pub fn stop_pg() -> Cmd {
        Cmd {
            run: format!("docker-compose stop {}", PG_SERVICE_ID),
            env: env::empty(),
            dir: Dir::Root,
            msg: "Stopping Postgres service",
        }
    }

    pub async fn pg_status() -> Result<ServiceStatus, io::Error> {
        let cmd = "docker-compose ps --services --filter status=running";

        Command::new(Cmd::SHELL)
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
