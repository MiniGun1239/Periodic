use crate::output::regular;

static CATEGORY_KEYWORDS: [&str; 3] = [
    "c", "cat", "category"
];

static GROUP_KEYWORDS: [&str; 3]    = [
    "g", "grp", "group"
];

static PERIOD_KEYWORDS: [&str; 4]   = [
    "p", "pd", "per", "period"
];

static BLOCK_KEYWORDS: [&str; 4]    = [
    "b", "bk", "blk", "block"
];


pub fn parse(number:u8, args: String) {
    if args == "h" || args == "help" || args == "-h" || args == "--help" {
        help()
    }

    else if args == "." || args == "a" || args == "all" {
        all(number);
    }

    else if CATEGORY_KEYWORDS.contains(&&args[..]) {
        category(number);
    }

    else if GROUP_KEYWORDS.contains(&&args[..]) {
        group(number);
    }

    else if PERIOD_KEYWORDS.contains(&&args[..]) {
        period(number);
    }

    else if BLOCK_KEYWORDS.contains(&&args[..]) {
        block(number);
    }

    else {
        incomplete();
    }
}

fn category(number: u8) {
    regular::grouping::category(number);
}

fn group(number: u8) {
    regular::grouping::group(number);
}

fn period(number: u8) {
    regular::grouping::period(number);
}

fn block(number: u8) {
    regular::grouping::block(number);
}


// Special Funcs


fn help() {
    println!("List of Available Options:");
    println!("Usage: periodic <ATOMIC_NUMBER> --grouping [OPTIONS]");
    println!(" ---");

    print!  ("  h, help         Prints help information\n\n");

    print!  ("  c, category     Prints category of element\n\n");

    println!("  g, group        Prints group of element");
    print!  ("  p, period       Prints period of element\n\n");

    println!("  b, block        Prints block of element");
}

fn incomplete() {
    println!("  Incorrect or no arguments provided");
    println!("   -- -- -- -- -- -- -- -- -- -- -- ");
    println!();

    help();
}

fn all(number: u8) {
    regular::grouping::all(number);
}
