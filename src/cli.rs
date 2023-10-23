use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(author = clap::crate_authors!())]
#[clap(color = clap::ColorChoice::Never)]
#[clap(about = clap::crate_description!(), long_about = clap::crate_description!())]
pub struct Cli {
    #[clap(
        help = "A positive integer between 0 and 4294967295",
        long_help = "A positive integer between 0 and 4294967295",
        required = true,
        value_parser = clap::value_parser!(u32).range(2..)
    )]
    pub number: u32,
}
