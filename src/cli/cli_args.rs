use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about,
    long_about = None
)]
pub struct CliArgs {
    #[arg(
        default_missing_value = None,
    )]
    pub(crate) query: Option<String>,

    /// Print name of element
    #[arg(
        short = 'n',
        long = "name"
    )]
    pub name: bool,


    /// Print symbol of element
    #[arg(
        short = 's',
        long = "symbol",
    )]
    pub symbol: bool,


    /// Print mass of element
    #[arg(
        short = 'm',
        long = "atomic-mass",
        alias = "unified-mass",
        alias = "mass",
    )]
    pub mass: bool,


    /// Print info related to electrons
    #[arg(
        short = 'e',
        long = "electron",
        alias = "electronic-config"
    )]
    pub electron: Option<String>,


    /// Print information about physical properties
    #[arg(
        short = 'p',
        long = "physical-properties",
        alias = "phy",
        alias = "physical"

    )]
    pub physical_properties: Option<String>,


    /// Print grouping info
    #[arg(
        short = 'g',
        long = "grouping",
        alias = "periodic-table"
    )]
    pub grouping: Option<String>,


    /// Print periodic table
    #[arg(
        short = 't',
        long = "table"
    )]
    pub table: bool,


    /// Sort elements
    #[arg(
        long = "sort",
    )]
    pub sort: Option<String>,

    
    /// Reverses sort order
    #[arg(
        short = 'r',
        long = "reverse"
    )]
    pub reverse: bool,


    /// Swag
    #[arg(
        long = "logo"
    )]
    pub logo: bool,
}
