use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(
        alias = "atomic-number",
        alias = "an",
        alias = "number"
    )]
    pub number: Option<u8>,

    #[arg(
        long = "unified-mass",
        short = 'm',
        short_alias = 'u',
        alias = "um",
        alias = "am",
        alias = "mass"
    )]
    pub mass: bool,
}

pub fn parse_flags() -> CliArgs {
    let args = CliArgs::parse();
    args
}
