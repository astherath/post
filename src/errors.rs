use clap::{Error, ErrorKind};
use std::error::Error as ErrorTrait;
use std::{fmt, io};
pub type ClapError = Error;
pub fn handle_add_entry_error(error: impl std::fmt::Debug) -> ClapError {
    let kind = ErrorKind::Io;
    throw_clap_err(kind, &format!("{error:?}"))
}

pub fn check_index_bounds(index_wanted: &u16, len_of_stack: usize) -> Result<(), String> {
    if index_wanted >= &(len_of_stack as u16) {
        return Err(format!(
            "index {index_wanted} is outside of bounds of notes available ({len_of_stack})."
        ));
    }
    Ok(())
}
pub fn handle_view_error(error: impl std::fmt::Debug) -> ClapError {
    let kind = ErrorKind::Io;
    throw_clap_err(kind, &format!("error viewing entries: {error:?}"))
}
pub fn handle_delete_error(error: impl std::fmt::Debug) -> ClapError {
    let kind = ErrorKind::Io;
    throw_clap_err(kind, &format!("error deleting entry: {error:?}"))
}

pub fn handle_clear_error(error: impl std::fmt::Debug) -> ClapError {
    let kind = ErrorKind::Io;
    throw_clap_err(kind, &format!("error clearing entries: {error:?}"))
}

fn throw_clap_err(kind: ErrorKind, error_str: &str) -> ClapError {
    Error::raw(kind, error_str)
}

#[derive(Debug)]
pub struct ClapIoError {
    pub desc: String,
}

impl fmt::Display for ClapIoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl ErrorTrait for ClapIoError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}

impl From<io::Error> for ClapIoError {
    fn from(err: io::Error) -> Self {
        Self {
            desc: format!("{err}"),
        }
    }
}

impl From<String> for ClapIoError {
    fn from(err: String) -> Self {
        Self { desc: err }
    }
}
