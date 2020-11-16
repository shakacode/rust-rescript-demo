#[macro_use]
extern crate lazy_static;

mod app;
mod cfg;
mod cmd;
mod env;
mod loc;
mod net;
mod printer;
mod result;
mod services;

use std::process;

use app::App;
use cfg::CFG;
use cmd::{Cmd, Exec, Process};
use env::Env;
use loc::{Dir, File};
use net::{HttpAddr, TcpAddr};
use result::{error, Error, Result};

#[tokio::main]
async fn main() {
    let app = App::new();

    match app.run().await {
        Ok(()) => {
            printer::print_done();
            process::exit(0);
        }
        Err(Error::Piped(status)) => match status.code() {
            Some(code) => {
                printer::print_non_zero_exit_code(code);
                process::exit(code)
            }
            None => {
                printer::print_warning("No exit code.");
                process::exit(1)
            }
        },
        Err(Error::Io(error)) => {
            printer::print_error(error);
            process::exit(1);
        }
        Err(Error::NothingToExecute { cmd }) => {
            printer::print_help(app.inner(), cmd);
            process::exit(1);
        }
    };
}
