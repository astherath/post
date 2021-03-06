use crate::{errors, file_io};
use std::path::{Path, PathBuf};

pub type OptionNum<'a> = &'a Option<u16>;
pub type HandleResult = Result<(), errors::ClapError>;

pub fn handle_post(text: &str, comment: Option<String>) -> HandleResult {
    if let Err(error) = file_io::add_entry(text, comment) {
        return Err(errors::handle_add_entry_error(error));
    }
    Ok(())
}

pub fn handle_view(
    top: OptionNum,
    tail: OptionNum,
    index: OptionNum,
    all: &bool,
    no_fmt: &bool,
) -> HandleResult {
    let resp = {
        if *all {
            file_io::view_all_entries(*no_fmt)
        } else if let Some(num) = top {
            file_io::view_entries_from_end(file_io::Range::Top(num), *no_fmt)
        } else if let Some(num) = tail {
            file_io::view_entries_from_end(file_io::Range::Tail(num), *no_fmt)
        } else if let Some(num) = index {
            file_io::view_entry_by_index(num, *no_fmt)
        } else {
            let default_amount_of_notes_to_view = file_io::Range::Top(&10);
            file_io::view_entries_from_end(default_amount_of_notes_to_view, *no_fmt)
        }
    };
    if let Err(error) = resp {
        return Err(errors::handle_view_error(error));
    }
    Ok(())
}
pub fn handle_clear(all: &bool, top: OptionNum, tail: OptionNum) -> HandleResult {
    let resp = {
        if *all {
            file_io::clear_all_entries()
        } else if let Some(num) = top {
            file_io::clear_from_end(file_io::Range::Top(num))
        } else if let Some(num) = tail {
            file_io::clear_from_end(file_io::Range::Tail(num))
        } else {
            unreachable!();
        }
    };
    if let Err(error) = resp {
        return Err(errors::handle_clear_error(error));
    }
    Ok(())
}
pub fn handle_pop(index: &u16) -> HandleResult {
    match file_io::handle_pop_entry(index) {
        Ok(_) => Ok(()),
        Err(error) => Err(errors::handle_pop_error(error)),
    }
}
pub fn handle_backup(path: &Path) -> HandleResult {
    let mut dest_path = PathBuf::from(path);
    match file_io::backup_data_file(&mut dest_path) {
        Ok(_) => Ok(()),
        Err(error) => Err(errors::handle_backup_error(error)),
    }
}
pub fn handle_delete(index: &u16) -> HandleResult {
    match file_io::delete_entry_from_file_by_index(index) {
        Ok(_) => Ok(()),
        Err(error) => Err(errors::handle_delete_error(error)),
    }
}
pub fn handle_yank(index: &u16) -> HandleResult {
    if let Err(error) = file_io::yank_note(index) {
        return Err(errors::handle_yank_error(error));
    }
    Ok(())
}
