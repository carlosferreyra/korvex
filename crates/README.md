# Crates

Korvex is structured as a Cargo workspace. Each crate has a focused responsibility. This file is the
authoritative glossary.

## `korvex`

The top-level SDK library. Downstream consumers depend on this crate. It re-exports the public API
composed from `korvex-macros` and `korvex-core`, using the shared types from `korvex-types`.

## `korvex-cli`

CLI logic and binary entrypoint. Structured as a library (`lib.rs`) containing all command
implementations, with a thin binary at `src/bin/korvex.rs` that calls `korvex_cli::run()`. This
mirrors the uv pattern — the entrypoint can be moved to `crates/korvex/src/bin/` later without
touching any logic. Provides commands for inspecting annotated interfaces, validating annotations,
and running the code generation pipeline from the terminal or a CI/CD environment.

## `korvex-types`

The shared data model. Defines the intermediate representation (IR) of Rust types and interfaces
that all other crates read and write. No other crate in this workspace should define its own IR
types.

## `korvex-macros`

The proc-macro crate. Exposes `#[korvex::export]` and related attributes. Annotated types are
introspected at compile time and their IR is fed into `korvex-core`. Must remain a dedicated crate
(`proc-macro = true`).

## `korvex-adapter`

The adapter contract. Defines the `BindingAdapter` trait that every language adapter must implement.
Third-party crates can depend solely on this crate to ship their own adapter without touching the
engine or the SDK.

## `korvex-core`

The binding engine. Takes the IR from `korvex-types` and dispatches to the appropriate language
adapter via `BindingAdapter`. Both the proc-macro expansion path and the CLI call into this crate —
there is one engine, two entry points.

## `korvex-python`

Python adapter. Reads the Korvex IR and emits idiomatic [pyo3](https://pyo3.rs)-based binding glue.
Enabled via `features = ["python"]`.

## `korvex-node`

Node.js adapter. Reads the Korvex IR and emits idiomatic [napi-rs](https://napi.rs)-based binding
glue. Enabled via `features = ["node"]`.

## `korvex-wasi`

WASI adapter. Reads the Korvex IR and emits WIT interface definitions and wasm component model glue.
Targets `wasm32-wasip1/p2` today; designed for incremental `wasip3`/`wasip4` support. Enabled via
`features = ["wasi"]`.

## `xtask`

Dev automation. Not published to crates.io. Invoked via `cargo xtask <cmd>` using the alias in
`.cargo/config.toml`. Three checkpoint commands that form a pyramid — each includes the previous:

| Command             | What it runs                                                |
| ------------------- | ----------------------------------------------------------- |
| `cargo xtask check` | `cargo fmt --all --check` + `check` + `clippy -D warnings` |
| `cargo xtask test`  | check + `cargo test --workspace`                            |
| `cargo xtask build` | test + `cargo build --release` + `cargo install`            |
