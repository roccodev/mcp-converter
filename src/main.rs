/*
 * Main executable for the CLI.
*/

extern crate regex;
extern crate libc;

mod cli;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    cli::parse_args(&args);
}