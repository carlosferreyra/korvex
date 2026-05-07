# Korvex

Generate idiomatic language bindings from annotated Rust code.

Korvex is a zero-config Rust SDK: annotate your types and functions with `#[korvex::export]`, select
your target language via Cargo features, and Korvex generates the binding glue. No `.udl` files, no
separate schema — the source code is the contract.

Designed as a composable alternative to [UniFFI](https://github.com/mozilla/uniffi-rs), with an
adapter model that keeps each language backend independent from the core engine.

## Usage

```toml
# Cargo.toml
[dependencies]
korvex = { version = "0.1", features = ["python"] }
```

```rust
#[korvex::export]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

That's it. Korvex handles the rest.

## Target languages

| Feature  | Adapter         | Backed by                  |
| -------- | --------------- | -------------------------- |
| `python` | `korvex-python` | [pyo3](https://pyo3.rs)    |
| `node`   | `korvex-node`   | [napi-rs](https://napi.rs) |
| `wasi`   | `korvex-wasi`   | WIT + wasm component model |

## Crates

See [`crates/README.md`](crates/README.md) for the full glossary.

| Crate                                       | Description                                         |
| ------------------------------------------- | --------------------------------------------------- |
| [`korvex`](crates/korvex)                   | Public SDK — the crate consumers depend on          |
| [`korvex-cli`](crates/korvex-cli)           | CLI logic (lib) + `korvex` binary entrypoint        |
| [`korvex-adapter`](crates/korvex-adapter)   | `BindingAdapter` trait — implement to add a backend |
| [`korvex-core`](crates/korvex-core)         | Binding engine — IR processing and dispatch         |
| [`korvex-macros`](crates/korvex-macros)     | Proc-macro crate — `#[korvex::export]`              |
| [`korvex-types`](crates/korvex-types)       | Shared IR type definitions                          |
| [`korvex-python`](crates/korvex-python)     | Python adapter (pyo3)                               |
| [`korvex-node`](crates/korvex-node)         | Node.js adapter (napi-rs)                           |
| [`korvex-wasi`](crates/korvex-wasi)         | WASI adapter (WIT + wasm component model)           |
| [`xtask`](crates/xtask)                     | Dev automation — `cargo xtask check/test/build`     |

## Custom adapters

Every built-in backend (`korvex-python`, `korvex-node`, `korvex-wasi`) implements the
`BindingAdapter` trait from `korvex-adapter`. You can ship your own backend by depending only on
that crate — no need to touch the engine or the SDK.

```toml
[dependencies]
korvex-adapter = { version = "0.0.1" }
```

```rust
use korvex_adapter::BindingAdapter;

pub struct MyBinding;

impl BindingAdapter for MyBinding {
    fn generate_bridge(&self) {
        // your codegen logic here
    }
}
```

## CLI

Install the `korvex` binary:

```sh
cargo install korvex
```

Available commands:

```sh
korvex inspect   # show what bindings would be generated
korvex check     # validate annotations without generating
korvex generate  # write binding files to disk for packaging
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
