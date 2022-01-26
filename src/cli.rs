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

fn get_matches(cli: &Cli) {
    match &cli.command {
        Commands::Post { text } => {}
        Commands::View { top, tail } => {}
        Commands::Clear { all, top, tail } => {}
        Commands::Pop { index } => {}
        Commands::Yank { index } => {}
        Commands::Delete { index } => {}
    }
}
