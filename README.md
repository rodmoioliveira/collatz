# collatz

collatz is a CLI for printing the [hailstone sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) of a NUMBER.

[![Build status](https://github.com/rodmoioliveira/collatz/workflows/ci/badge.svg)](https://github.com/rodmoioliveira/collatz/actions)

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

Prints the hailstone sequence of a NUMBER

Usage: collatz <NUMBER>

Arguments:
  <NUMBER>
          A positive integer between 0 and 4294967295

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
