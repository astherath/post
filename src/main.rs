#![feature(never_type)]
mod cli;
mod errors;
mod file_io;
fn main() {
    cli::run_main();
}
