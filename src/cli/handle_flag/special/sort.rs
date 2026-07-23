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

    else if NAME_KEYWORDS.contains(&&*args) {
        name(reverse)
    }

    else if SYMBOL_KEYWORDS.contains(&&*args) {
        symbol(reverse)
    }

    else if NUMBER_KEYWORDS.contains(&&*args) {
        number(reverse)
    }

    else if MASS_KEYWORDS.contains(&&*args) {
        mass(reverse)
    }

    else {
        incomplete()
    }
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

fn help() {
    println!("List of Available Options:");
    println!("Usage: periodic --sort [OPTIONS]");
    println!("or");
    println!("Usage: periodic --sort --reverse [OPTIONS]");
    println!(" ---");

    print!  ("  h, help                 Prints help information\n\n");

    println!("     name                 Sorts by name alphabetically");
    println!("  s, symbol               Sorts by symbol alphabetically\n");

    println!("  n, number               Sorts by atomic number in ascending order");
    println!("  m, mass                 Sorts by atomic mass in ascending order");
}

fn incomplete() {
    println!("Incorrect or no arguments provided");
    println!(" -- -- -- -- -- -- -- -- -- -- -- ");
    println!();

    help();
}
