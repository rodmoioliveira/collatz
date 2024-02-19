#!/bin/bash

declare TRACE
[[ "${TRACE}" == 1 ]] && set -o xtrace
set -o errexit
set -o nounset
set -o pipefail

readme() {
  cat <<EOF >README.md
# collatz

collatz is a CLI for printing the [hailstone sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) of a number.

[![Build status](https://github.com/rodmoioliveira/collatz/workflows/ci/badge.svg)](https://github.com/rodmoioliveira/collatz/actions)
[![GitHub Release](https://img.shields.io/github/v/release/rodmoioliveira/collatz)](https://github.com/rodmoioliveira/collatz/releases)

## Installation

Archives of [precompiled binaries](https://github.com/rodmoioliveira/collatz/releases)
for \`collatz\` are available for Windows, macOS and Linux.

## Building

\`collatz\` is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/tools/install)
in order to compile it. To build \`collatz\`, run:

\`\`\`
git clone git@github.com:rodmoioliveira/collatz.git
cd collatz
make install
\`\`\`

## Usage

\`\`\`
cargo run -- --help

$(cargo run -- --help)
\`\`\`
EOF

  sd '(make\[1\]:.+\n)' '' README.md
  sd 'cargo run --' 'collatz' README.md
}

trap readme EXIT
