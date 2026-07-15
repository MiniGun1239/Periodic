pub mod details;
pub mod cli;
pub mod output;
pub mod electron;

use crate::electron::quantum_model::configuration::{get_full, get_semantic};

fn main() {
    let args = cli::parse::parse_flags();

    if let Some(num) = args.query {
        println!("Returning for Atomic Number: {}", num);

        if args.mass {
            println!("Mass of Atom: \x1b[1m{} ({})\x1b[0m = {}", output::name::output(num), num, output::mass::output(num))
        }

        if let Some(_string) = args.electron {
            println!("Number of Electrons: {:?}", output::electron::number(num));

            println!("Quantum Electronic Configuration: {}\n", get_full(num));

            println!("Shorthand Configuration: {}", get_semantic(num));
        }
    }

    return;
}
