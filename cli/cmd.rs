use std::{
    io,
    process::Stdio,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use console::Color;
use nix::{errno::Errno, sys::signal::Signal, unistd::Pid, Error as NixError};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    signal,
    sync::Mutex,
    task, time,
};

use crate::{error, printer, Dir, Env, Error, Result, TcpAddr};

#[derive(Clone)]
pub struct Cmd {
    pub run: String,
    pub env: Env,
    pub dir: Dir,
    pub msg: &'static str,
}

impl Cmd {
    #[cfg(unix)]
    pub const SHELL: &'static str = "/bin/sh";

    #[cfg(windows)]
    pub const SHELL: &'static str = "cmd";

    #[cfg(unix)]
    pub fn shelled(cmd: &str) -> Vec<&str> {
        vec!["-c", &cmd]
    }

    #[cfg(windows)]
    pub fn shelled(cmd: &str) -> Vec<&str> {
        vec!["/c", &cmd]
    }
}

enum ProcessStatus {
    Ready,
    Running(Arc<Mutex<Child>>),
}

pub enum ProcessOutput {
    // Visible,
    Hidden,
}

pub struct Process {
    tag: &'static str,
    cmd: Cmd,
    status: ProcessStatus,
}

impl Process {
    pub fn new(tag: &'static str, cmd: Cmd) -> Self {
        Process {
            tag,
            cmd,
            status: ProcessStatus::Ready,
        }
    }

    pub fn timeout() -> Duration {
        Duration::from_secs(10)
    }

    pub fn tag(&self) -> &str {
        self.tag
    }

    pub fn cmd(&self) -> &Cmd {
        &self.cmd
    }

    pub async fn up(&mut self, output: ProcessOutput) {
        let cmd = &self.cmd;

        println!(
            "❯ {} {}",
            console::style(format!("{}...", cmd.msg)).bold(),
            console::style(format!("$ {} @ {}", cmd.run, cmd.dir.display())).dim()
        );

        let stdio = match output {
            // ProcessOutput::Visible => Stdio::inherit,
            ProcessOutput::Hidden => Stdio::null,
        };

        let child = Command::new(Cmd::SHELL)
            .args(Cmd::shelled(&cmd.run))
            .envs(cmd.env.to_owned())
            .current_dir(cmd.dir.loc())
            .stdout(stdio())
            .stderr(stdio())
            .spawn()
            .expect(&format!("Failed to spawn the process: {}", cmd.run));

        self.status = ProcessStatus::Running(Arc::new(Mutex::new(child)));
    }

    #[cfg(unix)]
    pub async fn down(&mut self) -> io::Result<()> {
        match self.status {
            ProcessStatus::Ready => Err(error::invalid_input("Process is not running")),
            ProcessStatus::Running(ref process) => {
                let mut process = process.lock().await;
                match process.id() {
                    Some(process_id) => {
                        let pid = Pid::from_raw(process_id as i32);
                        match nix::sys::signal::kill(pid, Signal::SIGINT) {
                            Ok(()) => {
                                let res = tokio::select! {
                                    res = process.wait() => Some(res),
                                    _ = time::sleep(Process::timeout()) => None,
                                };
                                match res {
                                    Some(Ok(_)) => Ok(()),
                                    Some(Err(error)) => {
                                        println!("IO error on SIGINT: {}", error);
                                        self.kill().await
                                    }
                                    None => {
                                        println!("Killing process after timeout");
                                        self.kill().await
                                    }
                                }
                            }
                            Err(NixError::Sys(Errno::EINVAL)) => {
                                println!("Invalid signal. Killing process {}", pid);
                                self.kill().await
                            }
                            Err(NixError::Sys(Errno::EPERM)) => Err(error::invalid_input(format!(
                                "Insufficient permissions to signal process {}",
                                pid
                            ))),
                            Err(NixError::Sys(Errno::ESRCH)) => Err(error::invalid_input(format!(
                                "Process {} does not exist",
                                pid
                            ))),
                            Err(error) => {
                                Err(error::invalid_input(format!("Unexpected error {}", error)))
                            }
                        }
                    }
                    None => Err(error::invalid_input("Process {} does not have an id")),
                }
            }
        }
    }

    #[cfg(windows)]
    pub async fn down(&mut self) -> io::Result<()> {
        match self.status {
            ProcessStatus::Stopped => Err(error::invalid_input("Process is not running")),
            ProcessStatus::Running(process) => self.kill(),
        }
    }

    pub async fn kill(&self) -> io::Result<()> {
        match self.status {
            ProcessStatus::Ready => Ok(()),
            ProcessStatus::Running(ref process) => {
                let mut process = process.lock().await;
                if let Ok(()) = process.kill().await {
                    process.wait().await?;
                }
                Ok(())
            }
        }
    }
}

pub struct Exec;

impl Exec {
    pub async fn cmd(cmd: Cmd) -> Result {
        println!(
            "❯ {} {}",
            console::style(format!("{}:", cmd.msg)).bold(),
            console::style(format!("$ {} @ {}", cmd.run, cmd.dir.display())).dim()
        );

        Command::new(Cmd::SHELL)
            .args(Cmd::shelled(&cmd.run))
            .envs(cmd.env.to_owned())
            .current_dir(cmd.dir.loc())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect(&format!("Failed to spawn the process: {}", cmd.run))
            .wait_with_output()
            .await
            .map_err(Error::from)
            .and_then(|res| {
                if res.status.success() {
                    Ok(())
                } else {
                    Err(Error::Piped(res.status))
                }
            })
    }

    pub async fn process(x: Process) -> Result {
        let cmd = x.cmd();

        println!(
            "❯ {} {}",
            console::style(format!("{}...", cmd.msg)).bold(),
            console::style(format!("$ {} @ {}", cmd.run, cmd.dir.display())).dim()
        );

        Command::new(Cmd::SHELL)
            .args(Cmd::shelled(&cmd.run))
            .envs(cmd.env.to_owned())
            .current_dir(cmd.dir.loc())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect(&format!("Failed to spawn the process: {}", cmd.run))
            .wait_with_output()
            .await
            .map_err(Error::from)
            .and_then(|res| {
                if res.status.success() {
                    Ok(())
                } else {
                    Err(Error::Piped(res.status))
                }
            })
    }

    pub async fn cmd_seq(cmds: Vec<Cmd>) -> Result {
        let mut iter = cmds.iter();
        while let Some(cmd) = iter.next() {
            Exec::cmd(cmd.to_owned()).await?;
        }
        Ok(())
    }

    pub async fn dependent_cmd(cmd: Cmd, mut process: Process, addr: TcpAddr) -> Result {
        let cmd_done = Arc::new(AtomicBool::new(false));
        let process_exited = Arc::new(AtomicBool::new(false));

        task::spawn({
            let cmd_done = cmd_done.clone();
            let process_exited = process_exited.clone();

            async move {
                process.up(ProcessOutput::Hidden).await;
                while !cmd_done.load(Ordering::Relaxed) {
                    time::sleep(Duration::from_millis(500)).await;
                }
                process.down().await.unwrap();
                process_exited.store(true, Ordering::Relaxed);
            }
        });

        addr.wait().await?;
        Exec::cmd(cmd).await?;
        cmd_done.store(true, Ordering::Relaxed);
        while !process_exited.load(Ordering::Relaxed) {}
        Ok(())
    }

    pub async fn pool(pool: Vec<Process>) -> Result {
        let pool_size = pool.len();
        let exited_processes = Arc::new(AtomicUsize::new(0));

        let tag_col_length = pool.iter().fold(0, |acc, process| {
            let len = process.tag().len();
            if len > acc {
                len
            } else {
                acc
            }
        });

        let colors = colors::make(pool_size as u8);
        let processes: Vec<(Process, Color)> = pool.into_iter().zip(colors).collect();

        let processes_list = processes
            .iter()
            .fold(String::new(), |acc, (process, color)| {
                let styled = console::style(process.tag().to_string()).fg(*color).bold();
                if acc == "" {
                    styled.to_string()
                } else {
                    format!("{}, {}", acc, styled)
                }
            });

        println!(
            "❯ {} {}",
            console::style("Running process pool:").bold(),
            processes_list
        );

        for (process, color) in processes {
            let exited_processes = exited_processes.clone();

            task::spawn(async move {
                let cmd = &process.cmd();
                let tag = {
                    let txt = process.tag();
                    let len = txt.len();
                    let pad = " ".repeat(if len < tag_col_length {
                        tag_col_length - len + 2
                    } else {
                        2
                    });
                    console::style(format!("{}{}|", txt, pad)).fg(color).bold()
                };

                println!(
                    "{tag} ❯ {msg} {cmd}",
                    tag = tag,
                    msg = console::style(format!("{}...", cmd.msg)).bold(),
                    cmd = console::style(format!("$ {} @ {}", cmd.run, cmd.dir.display())).dim()
                );

                let mut child = Command::new(Cmd::SHELL)
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

                let res = child.wait_with_output().await;

                match res {
                    Ok(output) => match output.status.code() {
                        Some(0) => {
                            printer::print_info(&format!("Process {} exited with code 0.", tag,))
                        }
                        Some(code) => printer::print_warning(&format!(
                            "Process {} exited with non-zero code: {}",
                            tag, code
                        )),
                        None => {
                            printer::print_info(&format!("Process {} exited without code.", tag,))
                        }
                    },
                    Err(error) => printer::print_error(error::other(format!(
                        "Process {} exited with error: {}",
                        tag, error
                    ))),
                }
                exited_processes.fetch_add(1, Ordering::Relaxed);
            });
        }

        signal::ctrl_c().await.unwrap();

        let expire = Instant::now() + Process::timeout();
        while exited_processes.load(Ordering::Relaxed) < pool_size {
            if Instant::now() > expire {
                printer::print_warning("Timeout. Exiting.");
                // TODO: Send SIGKILL to all processes on Unix (on Windows - ?)
                break;
            }
            time::sleep(Duration::from_millis(500)).await;
        }

        Ok(())
    }
}

mod colors {
    use console::Color;
    use rand::{seq::SliceRandom, thread_rng};

    pub fn make(n: u8) -> Vec<Color> {
        // Preferred colors
        let mut primaries = vec![
            // Color::Red, // Red is for errors
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
        ];
        // Not as good as primaries, but good enough to distinct processes
        let secondaries = vec![
            Color::Color256(24),
            Color::Color256(172),
            Color::Color256(142),
        ];

        // Let's check first if we can get away with just primary colors
        if n <= primaries.len() as u8 {
            shuffle(primaries, n)
        }
        // Otherwise, let's check if primary + secondary combined would work
        else if n <= (primaries.len() + primaries.len()) as u8 {
            primaries.extend(secondaries);
            shuffle(primaries, n)
        } else {
            // TODO: Duplicate primary + secondary colors vec as many is needed, then shuffle
            unimplemented!()
        }
    }

    fn shuffle<T>(mut items: Vec<T>, n: u8) -> Vec<T> {
        items.truncate(n as usize);
        items.shuffle(&mut thread_rng());
        items
    }
}
