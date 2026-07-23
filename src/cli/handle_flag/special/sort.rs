use crate::output;

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
