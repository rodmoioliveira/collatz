#!/bin/bash

declare TRACE
[[ "${TRACE}" == 1 ]] && set -o xtrace
set -o errexit
set -o nounset
set -o pipefail
shopt -s inherit_errexit

index() {
  paste -d "" \
    <(
      cat dev/readme.sh |
        grep -E '^#{1,} [A-Z]' |
        sed 's/^ {1,}//g' |
        sed -E 's/(^#{1,}) (.+)/\1\[\2]/g' |
        sed 's/#/  /g' |
        sed -E 's/\[/- [/g'
    ) \
    <(
      cat dev/readme.sh |
        grep -E '^#{1,} [A-Z]' |
        sed 's/#//g' |
        sed -E 's/^ {1,}//g' |
        sed -E 's/[?,]//g' |
        sed 's/[A-Z]/\L&/g' |
        sed 's/ /-/g' |
        sed -E 's@(.+)@(#\1)@g'
    )
}

backlink() {
  sed -i -E '/^#{1,} [A-Z]/a\\n\[back^\](#index)' README.md
}

readme() {
  cat <<EOF >README.md
# collatz

collatz is a CLI for printing the [hailstone sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) of a number.

[![Build status](https://github.com/rodmoioliveira/collatz/workflows/ci/badge.svg)](https://github.com/rodmoioliveira/collatz/actions)
[![GitHub Release](https://img.shields.io/github/v/release/rodmoioliveira/collatz)](https://github.com/rodmoioliveira/collatz/releases)

# index

$(index)

# Installation

Archives of [precompiled binaries](https://github.com/rodmoioliveira/collatz/releases)
for \`collatz\` are available for Windows, macOS and Linux.

# Building

\`collatz\` is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/tools/install)
in order to compile it. To build \`collatz\`, run:

\`\`\`
git clone git@github.com:rodmoioliveira/collatz.git
cd collatz
make install
\`\`\`

# Usage

\`\`\`
cargo run -- --help

$(cargo run -- --help)
\`\`\`

# Make Recipes

\`\`\`
$(make help)
\`\`\`
EOF

  sed -i -E '/^make\[[0-9]/d' README.md
  sed -i -E 's/cargo run --/collatz/g' README.md
  backlink
}

trap readme EXIT
