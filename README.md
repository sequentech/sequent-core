# sequent-core
Shared code, like ballot encoder-decoder

# Generate javascript package

    export RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals'
    rustup run nightly-2022-04-07 wasm-pack build --out-name index --release --target web --features=wasmtest -- -Z build-std=panic_abort,std
    rustup run nightly-2022-04-07 wasm-pack pack .


# Run tests

    cargo test

# Generate JSON schema

    cargo build --release
    ./target/release/sequent-core > ballot-schema.json
