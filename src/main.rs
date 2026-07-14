pub mod elements;
pub mod config;

fn main() {
    let args = config::parse_flags();

    if let Some(num) = args.number {
        println!("Returning for Atomic Number: {}", num);
    }
}
