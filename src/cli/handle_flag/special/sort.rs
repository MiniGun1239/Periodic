use crate::cli::cli_args::CliArgs;

pub fn parse(cli_args: CliArgs, sort_args: String, reverse: bool) {
    todo!(
        "Parse everything"
    )
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
    todo!(
        "print help information"
    )
}

fn incomplete() {
    todo!(
        "print incomplete, then call help"
    )
}
