use crate::output;

static BOILING_KEYWORDS: [&str; 4] = [
    "b", "boil", "boiling", "boiling-point"
];

static MELTING_KEYWORDS: [&str; 4] = [
    "m", "melt", "melting", "melting-point"
];

static DENSITY_KEYWORDS: [&str; 2] = [
    "d", "density"
];

static PHASE_KEYWORDS: [&str; 4] = [
    "p", "phase", "state", "state-of-matter"
];

pub fn parse(number:u8, args: String) {
    if args == "h" || args == "help" || args == "-h" || args == "--help" {
        help()
    }

    else if args == "." || args == "a" || args == "all" {
        all(number);
    }

    else if BOILING_KEYWORDS.contains(&&args[..]) {
        boiling_point(number);
    }

    else if MELTING_KEYWORDS.contains(&&args[..]) {
        melting_point(number);
    }

    else if DENSITY_KEYWORDS.contains(&&args[..]) {
        density(number);
    }

    else if PHASE_KEYWORDS.contains(&&args[..]) {
        phase(number)
    }

    else {
        incomplete()
    }
}


// private funcs

fn boiling_point(number: u8) {
    output::physical::boil(number);
}

fn melting_point(number: u8) {
    output::physical::melt(number);
}

fn density(number: u8) {
    output::physical::density(number);
}

fn phase(number: u8) {
    output::physical::phase(number)
}


// special functions

fn help() {
    println!("List of Available Options:");
    println!("Usage: periodic <ATOMIC_NUMBER> --physical [OPTIONS]");
    println!(" ---");

    print  !("  h, help             Show this help message\n\n");

    println!("  b, boiling-point    Prints boiling point of element");
    println!("  m, melting-point    Prints melting point of element");

    println!("  d, density          Prints density of element at STP");
    println!("  p, phase            Prints phase of element at STP");
}

fn incomplete() {
    println!("  Incorrect or no arguments provided");
    println!("   -- -- -- -- -- -- -- -- -- -- -- ");

    help()
}

fn all(number: u8) {
    output::physical::all(number);
}
