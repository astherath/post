#![feature(never_type)]
mod cli;
mod file_io;
mod errors;
fn main() {
    cli::run_main();
}
