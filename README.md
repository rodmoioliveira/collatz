# collatz

collatz is a CLI for printing the [hailstone sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) of a number.

[![Build status](https://github.com/rodmoioliveira/collatz/workflows/ci/badge.svg)](https://github.com/rodmoioliveira/collatz/actions)
[![GitHub Release](https://img.shields.io/github/v/release/rodmoioliveira/collatz)](https://github.com/rodmoioliveira/collatz/releases)

# index

  - [Installation](#installation)
  - [Building](#building)
  - [Usage](#usage)
  - [Make Recipes](#make-recipes)

# Installation

[back^](#index)

Archives of [precompiled binaries](https://github.com/rodmoioliveira/collatz/releases)
for `collatz` are available for Windows, macOS and Linux.

# Building

[back^](#index)

`collatz` is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/tools/install)
in order to compile it. To build `collatz`, run:

```
git clone git@github.com:rodmoioliveira/collatz.git
cd collatz
make install
```

# Usage

[back^](#index)

```
collatz --help

Prints the hailstone sequence of a number

Usage: collatz <NUMBER>

Arguments:
  <NUMBER>
          A positive integer between 0 and 340282366920938463463374607431768211455

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

# Make Recipes

[back^](#index)

```
bash-all             Run all bash tests
bash-check           Check format bash code
bash-fmt             Format bash code
bash-lint            Check lint bash code
doc-changelog        Write CHANGELOG.mode
doc-readme           Write README.md
help                 Display this help screen
rs-audit             Audit Cargo.lock
rs-audit-fix         Update Cargo.toml to fix vulnerable dependency requirement
rs-build             Build binary
rs-cargo-deps        Install cargo dependencies
rs-check             Run check
rs-dev               Run check in watch mode
rs-doc               Open app documentation
rs-fix               Fix rust code
rs-fmt-fix           Format fix rust code
rs-fmt               Format rust code
rs-install           Install binary
rs-lint-fix          Fix lint rust code
rs-lint              Lint rust code
rs-outdated          Display when dependencies are out of date
rs-tests             Run tests
rs-uninstall         Uninstall binary
rs-update-cargo      Update dependencies
typos                Check typos
typos-fix            Fix typos
```
