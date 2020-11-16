use std::{io, process};

pub type Result = std::result::Result<Ok, Error>;

pub type Ok = ();

pub enum Error {
    Io(io::Error),
    Piped(process::ExitStatus),
    NothingToExecute { cmd: &'static [&'static str] },
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

pub mod error {
    use std::io::{self, ErrorKind};

    pub fn other(msg: impl ToString) -> io::Error {
        io::Error::new(ErrorKind::Other, msg.to_string())
    }

    pub fn invalid_input(msg: impl ToString) -> io::Error {
        io::Error::new(ErrorKind::InvalidInput, msg.to_string())
    }
}
