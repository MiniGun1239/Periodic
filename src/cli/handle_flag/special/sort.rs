use crate::cli::cli_args::CliArgs;

pub fn parse(cli_args: CliArgs, sort_args: String, reverse: bool) {
    todo!(
        "Parse everything"
    )
}

fn name(reverse: bool) {
    todo!(
        "call out_name, passing reverse"
    )
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
    todo!(
        "print help information"
    )
}

fn incomplete() {
    todo!(
        "print incomplete, then call help"
    )
}


// output functions

fn out_name(reverse: bool) {
    todo!(
        "call output::special::sort::name or name_reversed"
    )
}

fn out_symbol(reverse: bool) {
    todo!(
        "call output::special::sort::symbol or symbol_reversed"
    )
}

fn out_number(reverse: bool) {
    todo!(
        "call output::special::sort::number or number_reversed"
    )
}

fn out_mass(reverse: bool) {
    todo!(
        "call output::special::special::sort::mass_reverse"
    )
}
