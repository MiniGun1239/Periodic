use clap::Parser;
use crate::cli::args_struct::CliArgs;

pub fn parse_flags() -> CliArgs {
    let args: CliArgs = CliArgs::parse();
    args
}
