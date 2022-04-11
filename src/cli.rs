use crate::handlers::{self, HandleResult};
use clap::{AppSettings, ArgGroup, IntoApp, Parser, Subcommand};

pub fn run_main() {
    let args = Cli::parse();
    if let Err(clap_error) = handle_matches(&args) {
        let mut app = Cli::into_app();
        clap_error.format(&mut app).exit();
    }
}

fn handle_matches(cli: &Cli) -> HandleResult {
    match &cli.command {
        Commands::Add { text } => handlers::handle_post(text),
        Commands::View {
            top,
            tail,
            index,
            all,
        } => handlers::handle_view(top, tail, index, all),
        Commands::Clear { all, top, tail } => handlers::handle_clear(all, top, tail),
        Commands::Pop { index } => handlers::handle_pop(index),
        Commands::Yank { index } => handlers::handle_yank(index),
        Commands::Delete { index } => handlers::handle_delete(index),
    }
}

#[derive(Parser)]
#[clap(
    author,
    version,
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
    Add {
        /// The contents of the note
        text: String,
    },

    /// Views the notes in the stack (if no argument given, views the lates 10 notes)
    #[clap(
        group(
            ArgGroup::new("cmds")
            .required(false)
            .args(&["top", "tail", "index", "all"])))]
    View {
        /// If set, views ALL notes
        #[clap(long)]
        all: bool,
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
        /// Index of note to delete
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
