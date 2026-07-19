use std::process::exit;
use crate::cli::cli_args::CliArgs;

pub mod name;
pub mod symbol;
pub mod mass;
pub mod electron;
pub mod physical;
pub mod grouping;

pub fn handle(args: CliArgs) {
    if args.query != None {
        let number: u8 = args.query.unwrap();

        if number < 1 || number > 119 {
            println!("Number must be between 1 (H) and 119 (Uue)");
            exit(1);
        }

        println!("  Outputting for Element {}:", number);
        println!("---");

        if args.name {
            name::parse(number);
        }

        if args.symbol {
            symbol::parse(number);
        }

        if args.mass {
            mass::parse(number);
        }

        if args.electron != None {
            electron::parse(number, args.electron.unwrap().to_lowercase());
        }

        if args.grouping != None {
            grouping::parse(number, args.grouping.unwrap().to_lowercase());
        }

        if args.physical_properties != None {
            physical::parse(number, args.physical_properties.unwrap().to_lowercase())
        }

        exit(0);
    }
}
