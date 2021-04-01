# opinionated_rust_to_typescript

`opinionated_rust_to_typescript` is a Rust library for transpiling Rust code to
TypeScript.

* Build the docs: `rm -rf target/doc; cargo doc`
* Read the docs: `open target/doc/opinionated_rust_to_typescript/index.html`
* Run the tests: `cargo test`
* Delete cargo’s cache, if new code is being ignored: `cargo clean`
* Try an example: `cargo run --example transpile-arg -- "const FOUR: u8 = 4;"`