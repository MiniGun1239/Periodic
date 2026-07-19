use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(
        default_missing_value = None,
    )]
    pub(crate) query: Option<u8>,
    
    #[arg(
        short = 'n',
        long = "name"
    )]
    pub name: bool,

    #[arg(
        short = 's',
        long = "symbol",
    )]
    pub symbol: bool,

    #[arg(
        short = 'm',
        long = "atomic-mass",
        alias = "unified-mass",
        alias = "mass",
    )]
    pub mass: bool,

    #[arg(
        short = 'e',
        long = "electron",
        alias = "electronic-config",
        default_missing_value = None
    )]
    pub electron: Option<String>,

    #[arg(
        short = 'p',
        long = "physical-properties",
        alias = "phy",
        alias = "physical",
        default_value = "default"

    )]
    pub physical_properties: Option<String>,

    #[arg(
        short = 'g',
        long = "grouping",
        alias = "periodic-table"
    )]
    pub grouping: Option<String>,
}
