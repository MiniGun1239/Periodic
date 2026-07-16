use clap::Parser;
use crate::cli::cli_args::CliArgs;

pub fn parse_flags() -> CliArgs {
    let args: CliArgs = CliArgs::parse();
    args
}
