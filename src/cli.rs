use clap::{AppSettings, ArgGroup, Parser, Subcommand};

use std::ffi::OsString;
use std::path::PathBuf;

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(name = "post")]
#[clap(
    about = "A simple note taking tool",
    long_about = "a simple cli to keep and move notes in/out of the clipboard"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a note to the stack
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Post {
        /// The contents of the note
        text: String,
    },

    /// Views the notes in the stack
    #[clap(setting(AppSettings::ArgRequiredElseHelp),
        group(
            ArgGroup::new("cmds")
            .required(false)
            .args(&["top", "tail", "index"])))]
    View {
        /// Index of note to view
        #[clap(long, required = false)]
        index: Option<u16>,
        /// Amount of notes to view, starting from the latest note added
        #[clap(long, required = false)]
        top: Option<u16>,
        /// Amount of notes to view, starting from the oldest note added
        #[clap(long, required = false)]
        tail: Option<u16>,
    },
    /// Copies the text from a note onto the clipboard
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Yank {
        /// Index of note to yank (if not set, yanks the latest note)
        #[clap(required = false)]
        index: u16,
    },

    /// Yanks the contents of a note and then deletes it
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Pop {
        /// Index of note to pop (if not set, pops the latest note)
        #[clap(required = false)]
        index: u16,
    },
    /// Deletes a note
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Delete {
        /// Index of note to delete (if not set, deletes the latest note)
        #[clap(required = false)]
        index: u16,
    },
    /// Deletes many notes at once
    #[clap(setting(AppSettings::ArgRequiredElseHelp),
        group(
            ArgGroup::new("cmds")
            .required(false)
            .args(&["top", "tail", "all"])))]
    Clear {
        #[clap(long)]
        all: bool,
        /// Amount of notes to clear, starting from the latest note added
        #[clap(long, required = false)]
        top: Option<u16>,
        /// Amount of notes to clear, starting from the oldest note added
        #[clap(long, required = false)]
        tail: Option<u16>,
    },
}

pub fn run_main() {
    let args = Cli::parse();
    let matches = get_matches(&args);
}

type OptionNum<'a> = &'a Option<u16>;

fn handle_post(text: &str) {
    if let Err(error) = file_io::add_entry(text) {
        errors::handle_add_entry_error(error);
    }
}
fn handle_view(top: OptionNum, tail: OptionNum, index: OptionNum) {
    let resp = {
        if let Some(num) = top {
            file_io::view_entries_from_end(file_io::Range::Top(num))
        } else if let Some(num) = tail {
            file_io::view_entries_from_end(file_io::Range::Tail(num))
        } else if let Some(num) = index {
            file_io::view_entry_by_index(num)
        } else {
            unreachable!()
        }
    };
    if let Err(error) = resp {
        errors::handle_view_error(error);
    }
}
fn handle_clear(all: &bool, top: OptionNum, tail: OptionNum) {}
fn handle_pop(index: &u16) {}
fn handle_delete(index: &u16) {}
fn handle_yank(index: &u16) {}

fn get_matches(cli: &Cli) {
    match &cli.command {
        Commands::Post { text } => handle_post(text),
        Commands::View { top, tail, index } => handle_view(top, tail, index),
        Commands::Clear { all, top, tail } => handle_clear(all, top, tail),
        Commands::Pop { index } => handle_pop(index),
        Commands::Yank { index } => handle_yank(index),
        Commands::Delete { index } => handle_delete(index),
    }
}

mod file_io {
    use super::errors;
    use std::io;
    type DefaultResult = io::Result<()>;
    pub fn add_entry(text: &str) -> DefaultResult {
        write_text_to_file(text)?;
        Ok(())
    }

    fn print_entry_to_console(entry: &Entry) {}

    pub enum Range<'a> {
        Tail(&'a u16),
        Top(&'a u16),
    }

    pub fn view_entries_from_end(range: Range) -> DefaultResult {
        let entries = get_entries_from_file()?.entries;
        let print_n_to_console = |x: Vec<Entry>, num: &u16| {
            x.iter()
                .take(*num as usize)
                .for_each(|y| print_entry_to_console(y))
        };
        match range {
            Range::Top(num) => print_n_to_console(entries, num),
            Range::Tail(num) => print_n_to_console(entries.into_iter().rev().collect(), num),
        }
        Ok(())
    }

    pub fn view_entry_by_index(index: &u16) -> DefaultResult {
        let entries = get_entries_from_file()?.entries;
        errors::check_index_bounds(index, entries.len()).unwrap();
        let entry = entries.iter().find(|x| &x.index == index).unwrap();
        print_entry_to_console(&entry);
        Ok(())
    }

    pub fn view_entries() -> DefaultResult {
        let entries = get_entries_from_file()?;
        Ok(())
    }

    fn delete_entry_from_file(entry: &Entry) -> DefaultResult {}

    fn overwrite_entries_to_file(entries: Entries) -> DefaultResult {}

    fn write_text_to_file(text: &str) -> DefaultResult {
        Ok(())
    }
    fn get_entries_from_file() -> io::Result<Entries> {
        Ok(Entries::default())
    }
    struct Entries {
        pub entries: Vec<Entry>,
    }
    impl Default for Entries {
        fn default() -> Self {
            Self { entries: vec![] }
        }
    }
    struct Entry {
        content: String,
        index: u16,
    }
}

mod errors {
    pub fn handle_add_entry_error(error: impl std::fmt::Debug) -> ! {
        throw_clap_err(&format!("errored out with: {error:?}"));
    }

    pub fn handle_index_too_large_error(index: &u16, max_index: &u16) -> ! {
        throw_clap_err(&format!(
            "requested index too large for stack, wanted: {index} but max is: {}.",
            max_index - 1
        ));
    }

    pub fn check_index_bounds(index_wanted: &u16, len_of_stack: usize) -> Result<(), !> {
        if index_wanted >= &(len_of_stack as u16) {
            handle_index_too_large_error(index_wanted, &(len_of_stack as u16));
        }
        Ok(())
    }

    pub fn handle_view_error(error: impl std::fmt::Debug) -> ! {
        throw_clap_err(&format!("error viewing entries: {error:?}"));
    }

    fn throw_clap_err(error_str: &str) -> ! {
        panic!("{}", error_str);
    }
}
