pub mod details;
pub mod cli;
pub mod output;
pub mod electron;

use crate::cli::handle_flag::pass_to_handler;
use crate::cli::parse::parse_flags;

fn main() {
    let args = parse_flags();
    
    pass_to_handler(args);

    return;
}
