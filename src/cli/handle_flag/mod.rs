use crate::cli::cli_args::CliArgs;

pub mod name;
pub mod symbol;
pub mod mass;
pub mod electron;

pub fn handle(args: CliArgs) {
    if args.query == None {
        todo!()
    }

    let number: u8 = args.query.unwrap();

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
}
