# Korvex

**Kernel Orchestrated Rust Vector EXecutor**

> The Kernel-grade Orchestrator for Seamless Polyglot Execution.

Korvex is a Rust-first SDK for generating idiomatic language bindings from a single source of truth. It is designed as a modern, composable alternative to [UniFFI](https://github.com/mozilla/uniffi-rs), with first-class support for multiple target languages and a clean workspace-based architecture.

## Crates

See [`crates/README.md`](crates/README.md) for the full glossary.

| Crate | Description |
|---|---|
| [`korvex`](crates/korvex) | Core SDK library |
| [`korvex-cli`](crates/korvex-cli) | Command-line interface |
| [`korvex-types`](crates/korvex-types) | Shared type definitions |
| [`korvex-ffi`](crates/korvex-ffi) | FFI / ABI bridge layer |
| [`korvex-gen`](crates/korvex-gen) | Code generation engine |

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
