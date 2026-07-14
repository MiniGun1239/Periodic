pub mod details;
pub mod cli;
pub mod output;

fn main() {
    let args = cli::config::parse_flags();

    if let Some(num) = args.query {
        println!("Returning for Atomic Number: {}", num);

        if args.mass {
            println!("Mass of Atom: \x1b[1m{} ({})\x1b[0m = {}", output::name::output(num), num, output::mass::output(num))
        }
    }
}
