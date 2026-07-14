use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(
        long = "atomic-number",
        short = 'n',
        alias = "an",
        alias = "number"
    )]
    pub number: u8,

    
}
