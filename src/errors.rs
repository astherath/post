use clap::{Error, ErrorKind};
use std::error::Error as ErrorTrait;
use std::{fmt, io};
pub type ClapError = Error;
pub fn handle_add_entry_error(error: impl std::fmt::Debug) -> ClapError {
    let kind = ErrorKind::Io;
    throw_clap_err(kind, &format!("{error:?}"))
}

pub fn handle_index_too_large_error(index: &u16, max_index: &u16) -> ClapError {
    let kind = ErrorKind::InvalidValue;
    throw_clap_err(
        kind,
        &format!(
            "requested index too large for stack, wanted: {index} but max is: {}.",
            max_index - 1
        ),
    )
}

pub fn check_index_bounds(index_wanted: &u16, len_of_stack: usize) -> Result<(), ClapError> {
    if index_wanted >= &(len_of_stack as u16) {
        handle_index_too_large_error(index_wanted, &(len_of_stack as u16));
    }
    Ok(())
}
pub fn handle_entry_from_str_error(error: impl std::fmt::Debug) -> ClapError {
    let kind = ErrorKind::Io;
    throw_clap_err(
        kind,
        &format!("error parsing data from file, check that data is valid: {error:?}"),
    )
}
pub fn handle_view_error(error: impl std::fmt::Debug) -> ClapError {
    let kind = ErrorKind::Io;
    throw_clap_err(kind, &format!("error viewing entries: {error:?}"))
}
pub fn handle_delete_error(error: impl std::fmt::Debug) -> ClapError {
    let kind = ErrorKind::Io;
    throw_clap_err(kind, &format!("error deleting entry: {error:?}"))
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
