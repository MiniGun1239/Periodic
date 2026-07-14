pub mod details;
pub mod cli;
pub mod output;

fn main() {
    let args = cli::config::parse_flags();

    if let Some(num) = args.query {
        println!("Returning for Atomic Number: {}", num);
        println!("{}", output::mass::output(num));
    }
}
