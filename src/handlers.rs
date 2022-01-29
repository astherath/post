use crate::{errors, file_io};

pub type OptionNum<'a> = &'a Option<u16>;
pub type HandleResult = Result<(), errors::ClapError>;

pub fn handle_post(text: &str) -> HandleResult {
    if let Err(error) = file_io::add_entry(text) {
        return Err(errors::handle_add_entry_error(error));
    }
    Ok(())
}

pub fn handle_view(top: OptionNum, tail: OptionNum, index: OptionNum) -> HandleResult {
    let resp = {
        if let Some(num) = top {
            file_io::view_entries_from_end(file_io::Range::Top(num))
        } else if let Some(num) = tail {
            file_io::view_entries_from_end(file_io::Range::Tail(num))
        } else if let Some(num) = index {
            file_io::view_entry_by_index(num)
        } else {
            unreachable!();
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
    Ok(())
}
pub fn handle_delete(index: &u16) -> HandleResult {
    match file_io::delete_entry_from_file_by_index(index) {
        Ok(_) => Ok(()),
        Err(error) => Err(errors::handle_delete_error(error)),
    }
}
pub fn handle_yank(index: &u16) -> HandleResult {
    Ok(())
}