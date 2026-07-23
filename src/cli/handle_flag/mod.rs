pub mod regular;
pub mod special;

use std::process::exit;
use crate::cli::cli_args::CliArgs;
use crate::output::special::logo;

pub fn pass_to_handler(args: CliArgs) {
    if args.logo {
        logo::ascii();
        exit(0);
    }

    if let Some(_query) = args.query.as_ref() {
        let query = _query.to_lowercase();

        if query == "table" {
            special::table::table();
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

        if args.sort != None {
            special::sort::help()
        } else if args.table {
            special::table::table();
        }

        handle(number, args);
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
        regular::name::parse(number);
    }

    if args.symbol {
        regular::symbol::parse(number);
    }

    if args.mass {
        regular::mass::parse(number);
    }

    if args.electron != None {
        regular::electron::parse(number, args.electron.unwrap().to_lowercase());
    }

    if args.grouping != None {
        regular::grouping::parse(number, args.grouping.unwrap().to_lowercase());
    }

    if args.physical_properties != None {
        regular::physical::parse(number, args.physical_properties.unwrap().to_lowercase())
    }

}

fn handle_no_num(args: CliArgs) {
    check_illegal_arguments(&args);

    if args.sort != None {
        special::sort::parse(args.sort.unwrap(), args.reverse);
    }

    if args.table {
        special::table::table();
    }
}

fn check_illegal_arguments(args: &CliArgs) {
    if args.name {
        println!("Usage: periodic <atomic_number> --name");
    }

    if args.symbol {
        println!("Usage: periodic <atomic_number> --symbol");
    }

    if args.mass {
        println!("Usage: periodic <atomic_number> --mass");
    }

    if args.electron != None {
        regular::electron::parse(0, "help".parse().unwrap());
    }

    if args.grouping != None {
        regular::grouping::parse(0, "help".parse().unwrap());
    }

    if args.physical_properties != None {
        regular::physical::parse(0, "help".parse().unwrap());
    }
}