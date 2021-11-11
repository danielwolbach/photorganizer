use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(name = "photorganizer", author = "Daniel Wolbach")]
pub struct Options {
    #[clap(short, long)]
    pub source: String,

    #[clap(short, long)]
    pub destination: String,

    #[clap(short, long)]
    pub parallel: bool,
}
