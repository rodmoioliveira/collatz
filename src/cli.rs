use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(author = clap::crate_authors!())]
#[clap(color = clap::ColorChoice::Never)]
#[clap(about = clap::crate_description!(), long_about = clap::crate_description!())]
pub struct Cli {
    #[clap(
        help = "A positive integer between 0 and 340282366920938463463374607431768211455",
        long_help = "A positive integer between 0 and 340282366920938463463374607431768211455",
        required = true
    )]
    pub number: u128,
}
