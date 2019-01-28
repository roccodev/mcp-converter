/*
 * Main executable for the CLI.
*/

mod cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    cli::parse_args(&args);
}