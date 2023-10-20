use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(author = clap::crate_authors!())]
#[clap(color = clap::ColorChoice::Never)]
#[clap(about = clap::crate_description!(), long_about = clap::crate_description!())]
pub struct Cli {
    #[clap(
        required = true,
        help = "A positive integer between 0 and u32::MAX",
        long_help = "A positive integer between 0 and 4294967295",
    )]
    pub number: u32,
}

// The Rust standard library suppresses the default SIGPIPE behavior, so that
// writing to a closed pipe doesn't kill the process.
//
// See:
// https://stackoverflow.com/questions/65755853/simple-word-count-rust-program-outputs-valid-stdout-but-panicks-when-piped-to-he
// https://github.com/BurntSushi/ripgrep/commit/3065a8c9c839f7e722a73e8375f2e41c7e084737
#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

fn main() {
    reset_sigpipe();
    let args = Cli::parse();

    if let 0..=1 = args.number {
        eprintln!("no steps");
        std::process::exit(0);
    };

    let mut next = args.number as u128;
    let mut step = 0;

    println!("next step");
    while next != 1 {
        step += 1;

        match next % 2 == 0 {
            true => {
                next /= 2;
            }
            false => {
                next = (next * 3) + 1;
            }
        }

        println!("{step} {next}");
    }
}
