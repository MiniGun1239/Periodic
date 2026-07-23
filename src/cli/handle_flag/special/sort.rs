use crate::output;

static  NAME_KEYWORDS: [&str; 3] = [
    "name", "by_name", "alphabetical"
];

static SYMBOL_KEYWORDS: [&str; 5] = [
    "symbol", "by_symbol",
    "sym", "by_sym",
    "s"
];

static NUMBER_KEYWORDS: [&str; 5] = [
    "number", "by_number",
    "num", "by_num",
    "n"
];

static MASS_KEYWORDS: [&str; 3] = [
    "mass", "by_mass",
    "m"
];


pub fn parse(args: String, reverse: bool) {
    if args == "h" || args == "help" {
        help()
    }

    todo!()
}

fn name(reverse: bool) {
    output::special::sort::name(reverse);
}

fn symbol(reverse: bool) {
    output::special::sort::symbol(reverse);
}

fn number(reverse: bool) {
    output::special::sort::number(reverse);
}

fn mass(reverse: bool) {
    output::special::sort::mass(reverse);
}


// special functions

pub(crate) fn help() {
    println!("List of Available Options:");

    todo!()
}

fn incomplete() {
    println!("Incorrect or no arguments provided");
    println!(" -- -- -- -- -- -- -- -- -- -- -- ");
    println!();

    help();
}
