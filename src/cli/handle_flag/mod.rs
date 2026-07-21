pub mod regular;
pub mod special;

use std::process::exit;
use regular::{electron, grouping, mass, name, physical, symbol};
use crate::cli::cli_args::CliArgs;
use crate::output::special::logo::ascii;

pub fn pass_to_handler(args: CliArgs) {
    if args.logo {
        ascii();
        exit(0)
    }

    if let Some(_query) = args.query.as_ref() {
        let query = _query.to_lowercase();

        if query == "table" {
            todo!(
                "call special::table"
            )
        }

        let number: u8 = match query.parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid number");
                exit(1);
            }
        };

        if !(1..=119).contains(&number) {
            eprintln!("Number has to be between 1 (H) and 119 (Uue)");
        }

        handle(number, args)
    }

    else {
        handle_no_num(args);

        exit(0)
    }
}

fn handle(number: u8, args: CliArgs) {
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

}

fn handle_no_num(args: CliArgs) {
    let mut reverse: bool = false;

    if args.reverse {
        reverse = true;
    }

    if args.sort != None {
        if reverse {
            todo!(
                "call special::sort with reverse"
            )
        }
        else {
            todo!(
                "call special::sort"
            )
        }
    }

    if args.table {
        todo!(
            "call special::table"
        )
    }
}
