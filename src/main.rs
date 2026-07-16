pub mod details;
pub mod cli;
pub mod output;
pub mod electron;

use crate::electron::quantum_model::configuration::{get_full, get_semantic};

fn main() {
    let args = cli::parse::parse_flags();

    return;
}
