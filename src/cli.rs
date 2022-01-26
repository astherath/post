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
            .args(&["top", "tail"])))]
    View {
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

fn handle_adding_text(text: &str) {
    if let Err(error) = file_io::add_entry(text) {
        errors::handle_add_entry_error(error);
    }
}

type OptionNum<'a> = &'a Option<u16>;

fn handle_view(top: OptionNum, tail: OptionNum) {}

fn handle_clear(all: OptionNum, top: OptionNum, tail: OptionNum) {}

fn handle_pop(index: OptionNum) {}
fn handle_delete(index: u16) {}
fn handle_yank(index: OptionNum) {}

fn get_matches(cli: &Cli) {
    match &cli.command {
        Commands::Post { text } => handle_adding_text(text),
        Commands::View { top, tail } => handle_view(top, tail),
        Commands::Clear { all, top, tail } => handle_clear(all, top, tail),
        Commands::Pop { index } => handle_pop(index),
        Commands::Yank { index } => handle_yank(index),
        Commands::Delete { index } => handle_delete(index),
    }
}

mod file_io {
    use std::io;
    pub fn add_entry(text: &str) -> io::Result<()> {
        write_text_to_file(text)?;
        Ok(())
    }
    fn write_text_to_file(text: &str) -> io::Result<()> {
        Ok(())
    }
}

mod errors {
    pub fn handle_add_entry_error(error: impl std::fmt::Debug) -> ! {
        throw_clap_err(&format!("errored out with: {error:?}"))
    }

    fn throw_clap_err(error_str: &str) -> ! {
        panic!("{}", error_str);
    }
}
