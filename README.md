# collatz

collatz is a CLI for printing the [hailstone sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) of a number.

[![Build status](https://github.com/rodmoioliveira/collatz/workflows/ci/badge.svg)](https://github.com/rodmoioliveira/collatz/actions)
[![GitHub Release](https://img.shields.io/github/v/release/rodmoioliveira/collatz)](https://github.com/rodmoioliveira/collatz/releases)

## Installation

Archives of [precompiled binaries](https://github.com/rodmoioliveira/collatz/releases)
for `collatz` are available for Windows, macOS and Linux.

## Building

`collatz` is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/tools/install)
in order to compile it. To build `collatz`, run:

```
git clone git@github.com:rodmoioliveira/collatz.git
cd collatz
make install
```

## Usage

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
