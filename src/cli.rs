use super::{errors, file_io};
use clap::{AppSettings, ArgGroup, Parser, Subcommand};
#[derive(Parser)]
#[clap(
    name = "post",
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
fn handle_delete(index: &u16) {
    errors::handle_delete_error(file_io::delete_entry_from_file_by_index(index));
}
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
