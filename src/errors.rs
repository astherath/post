use clap::{Error, ErrorKind};
pub fn handle_add_entry_error(error: impl std::fmt::Debug) -> ! {
    let kind = ErrorKind::Io;
    throw_clap_err(kind, &format!("errored out with: {error:?}"));
}

pub fn handle_index_too_large_error(index: &u16, max_index: &u16) -> ! {
    let kind = ErrorKind::InvalidValue;
    throw_clap_err(
        kind,
        &format!(
            "requested index too large for stack, wanted: {index} but max is: {}.",
            max_index - 1
        ),
    );
}

pub fn check_index_bounds(index_wanted: &u16, len_of_stack: usize) -> Result<(), !> {
    if index_wanted >= &(len_of_stack as u16) {
        handle_index_too_large_error(index_wanted, &(len_of_stack as u16));
    }
    Ok(())
}
pub fn handle_entry_from_str_error(error: impl std::fmt::Debug) -> ! {
    let kind = ErrorKind::Io;
    throw_clap_err(
        kind,
        &format!("error parsing data from file, check that data is valid: {error:?}"),
    );
}
pub fn handle_view_error(error: impl std::fmt::Debug) -> ! {
    let kind = ErrorKind::Io;
    throw_clap_err(kind, &format!("error viewing entries: {error:?}"));
}
pub fn handle_delete_error(error: impl std::fmt::Debug) -> ! {
    let kind = ErrorKind::Io;
    throw_clap_err(kind, &format!("error deleting entry: {error:?}"));
}

fn throw_clap_err(kind: ErrorKind, error_str: &str) -> ! {
    let error = Error::raw(kind, error_str);
    error.print().unwrap();
    error.exit();
}
