use crate::cli::handle_flag::handle;

pub mod details;
pub mod cli;
pub mod output;
pub mod electron;

fn main() {
    let args = cli::parse::parse_flags();

    return;
}
