pub mod cli {
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
}

pub mod hailstone {
    use num::bigint::BigUint;
    use num::Integer;

    pub fn sequence(number: u128) -> Vec<BigUint> {
        if number == 0_u128 {
            return vec![];
        }

        let mut steps: Vec<BigUint> = Vec::with_capacity(64);
        let mut next: BigUint = BigUint::from(number);
        let zero = num::zero();
        let one = num::one();
        let two = &one + &one;
        let three = &two + &one;

        while next != one {
            match next.mod_floor(&two) == zero {
                true => {
                    next /= &two;
                }
                false => {
                    next = (&next * &three) + &one;
                }
            }
            steps.push(next.clone());
        }

        steps
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use num::bigint::BigUint;

        #[test]
        fn test_sequence() {
            let seq = sequence(10);
            let answer: [u128; 6] = [5, 16, 8, 4, 2, 1];

            assert_eq!(
                seq,
                answer
                    .into_iter()
                    .map(BigUint::from)
                    .collect::<Vec<BigUint>>()
            );
        }
    }
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

#[cfg(not(unix))]
fn reset_sigpipe() {}

fn main() {
    use clap::Parser;

    reset_sigpipe();
    let args = cli::Cli::parse();

    let seq = hailstone::sequence(args.number);
    println!("step next");
    for (i, n) in seq.iter().enumerate() {
        println!("{s} {n}", s = i + 1);
    }
}
