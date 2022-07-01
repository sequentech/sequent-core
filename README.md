<!--
SPDX-FileCopyrightText: 2022 Felix Robles <felix@sequentech.io>

SPDX-License-Identifier: AGPL-3.0-only
-->
# sequent-core

Sequent shared code. This code might be used in different projects/packages, like the ballot verifier or the voting booth.

Currently this includes:
 * The structures that represent an auditable ballot.
 * Methods to generate the ballot cyphertexts.
 * Methods to generate a hash from a cyphertext.

In the future this repo will also include the ballot encoder-decoder.

# Development environment

sequent-core uses [Nix Package Manager](https://nixos.org/) as its package builder. To build
new-ballot-verifier, **first [install Nix](https://nixos.org/)** correctly in your system.

After you have installed Nix, enter the development environment with:

```bash
nix develop
```

# Generate javascript package

    export RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals'
    rustup run nightly-2022-04-07 wasm-pack build --out-name index --release --target web --features=wasmtest -- -Z build-std=panic_abort,std
    rustup run nightly-2022-04-07 wasm-pack pack .


# Run tests

    cargo test

# Generate JSON schema

    cargo build --release
    ./target/release/sequent-core > ballot-schema.json

## Continuous Integration

There are multiple checks executed through the usage of Github Actions to verify
the health of the code when pushed:
1. **Compiler warning/errors**: checked using `cargo check` and 
`cargo check ---tests`. Use `cargo fix` and `cargo fix --tests` to fix the 
issues that appear.
2. **Unit tests**: check that all unit tests pass using `cargo test`.
3. **Code style**: check that the code style follows standard Rust format, using
`cargo fmt -- --check`. Fix it using `cargo fmt`.
4. **Code linting**: Lint that checks for common Rust mistakes using 
`cargo clippy`. You can try to fix automatically most of those mistakes using
`cargo clippy --fix -Z unstable-options`.
5. **Code coverage**: Detects code coverage with [grcov] and pushes the 
information (in master branch) to [codecov].
1. **License compliance**: Check using [REUSE] for license compliance within
the project, verifying that every file is REUSE-compliant and thus has a 
copyright notice header.
