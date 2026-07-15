use crate::electron::quantum_atomic_model::{get_full_config, get_shorthand_config};

pub mod details;
pub mod cli;
pub mod output;
pub mod electron;

fn main() {
    let args = cli::parse::parse_flags();

    if let Some(num) = args.query {
        println!("Returning for Atomic Number: {}", num);

        if args.mass {
            println!("Mass of Atom: \x1b[1m{} ({})\x1b[0m = {}", output::name::output(num), num, output::mass::output(num))
        }

        if let Some(_string) = args.electron {
            println!("Quantum Electronic Configuration: {}\n", get_full_config(num));

            println!("Shorthand Configuration: {}", get_shorthand_config(num));
        }
    }

    return;
}
