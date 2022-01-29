#![feature(never_type)]
mod cli;
mod errors;
mod file_io;
mod handlers;
fn main() {
    cli::run_main();
}
