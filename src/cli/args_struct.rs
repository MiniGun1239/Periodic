use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(
        alias = "q"
    )]
    pub(crate) query: Option<u8>,
    
    #[arg(
        short = 'm',
        long = "unified-mass",
        alias = "mass",
    )]
    pub mass: bool,
    
    #[arg(
        short = 'n',
        long = "name"
    )]
    pub name: bool,
    
    #[arg(
        short = 'e',
        long = "electron",
        alias = "electronic-config",
    )]
    pub electron: Option<String>
}
