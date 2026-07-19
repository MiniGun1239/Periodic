pub mod details;
pub mod cli;
pub mod output;
pub mod electron;

use crate::cli::handle_flag::handle;

fn main() {
    println!("---");

    let args = cli::parse::parse_flags();

    handle(args);

    return;
}
