# Crates

Korvex is structured as a Cargo workspace. Each crate has a focused responsibility. This file is the authoritative glossary.

## `korvex`

The top-level SDK library. Consumers depend on this crate directly. It re-exports the public API composed from `korvex-ffi` and `korvex-gen`, using the shared types from `korvex-types`.

## `korvex-cli`

The `korvex` binary. Provides commands for scaffolding new projects, inspecting interfaces, and invoking the code generation pipeline from the terminal.

## `korvex-types`

The shared data model. Defines the intermediate representation (IR) of Rust types and interfaces that all other crates read and write. No crate in this workspace except `korvex-types` should define its own IR types.

## `korvex-ffi`

The FFI and ABI bridge layer. Responsible for exposing Rust types across the C ABI boundary, generating `extern "C"` declarations, and managing memory ownership contracts between Rust and foreign runtimes.

## `korvex-gen`

The code generation engine. Takes the IR from `korvex-types` and emits idiomatic binding code for each target language (e.g. Python, Swift, Kotlin, Go). Each language backend is a module within this crate.
