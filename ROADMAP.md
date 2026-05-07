# Korvex Roadmap

This document defines the milestones to reach an MVP: a user annotates Rust code with
`#[korvex::export]`, runs `korvex generate`, and gets working PyO3 / napi-rs bridge code that can be
packaged and published to PyPI / npm.

## Macro syntax (unstable)

The `#[korvex::export]` attribute is the single annotation surface. It is **unstable** — args and
behavior may change before `0.1.0`. The MVP supports three forms:

| Annotation                 | Applied to                   | Meaning                                 |
| -------------------------- | ---------------------------- | --------------------------------------- |
| `#[korvex::export]`        | free function                | export as a callable function (default) |
| `#[korvex::export(class)]` | struct                       | export as a class                       |
| `#[korvex::export(new)]`   | constructor fn in impl block | mark the constructor for the class      |

`korvex check` enforces: every `class` must have exactly one `new`.

Post-MVP: `#[korvex::export(method)]` for additional methods on classes.

## CLI

```sh
korvex inspect                              # print IR as JSON for the current crate
korvex check                               # validate annotations; exit non-zero on errors
korvex generate --target python            # write PyO3 bridge to examples/python/ (default)
korvex generate --target python --output . # write to a custom path (production use)
korvex generate --target node              # write napi-rs bridge to examples/node/ (default)
korvex generate --target node --output .   # write to a custom path
```

`korvex` crate is the binary entrypoint; `korvex-cli` contains the logic.

## Type mapping

| Rust                       | Python      | TypeScript  |
| -------------------------- | ----------- | ----------- |
| `i32`, `i64`, `u32`, `u64` | `int`       | `number`    |
| `f32`, `f64`               | `float`     | `number`    |
| `bool`                     | `bool`      | `boolean`   |
| `String`, `&str`           | `str`       | `string`    |
| `Option<T>`                | `T \| None` | `T \| null` |
| `Vec<T>`                   | `list[T]`   | `T[]`       |

Other types (`HashMap`, `Result`, newtypes) are post-MVP.

---

## Milestone 0 — IR Foundation

> **Goal:** Define the shared type system that all other crates depend on.

- [ ] Define IR types in `korvex-types`:
  - `IrFunction` — name, params, return type
  - `IrClass` — name, fields, constructor
  - `IrConstructor` — params
  - `IrField` — name, type
  - `IrParam` — name, type
  - `IrType` — enum covering the MVP type mapping set
- [ ] `korvex inspect` CLI command: print IR as JSON for a given crate

**Done when:** `cargo xtask test` passes and `korvex inspect` prints valid JSON for an annotated
crate.

---

## Milestone 1 — Macro extraction

> **Goal:** `#[korvex::export]` reads annotated items and emits IR.

- [ ] `#[korvex::export]` on free functions → `IrFunction`
- [ ] `#[korvex::export(class)]` on structs → `IrClass`
- [ ] `#[korvex::export(new)]` on constructor impl fn → `IrConstructor`
- [ ] Proc-macro collects annotated items and serializes them to IR at compile time

**Done when:** `korvex inspect` correctly outputs IR for functions and classes in a sample crate.

---

## Milestone 2 — Python adapter (PyO3)

> **Goal:** Generate a Rust bridge file that maturin can compile into a Python-importable package.

- [ ] `PyO3Binding` reads IR and generates:
  - `#[pyfunction]` wrappers for free functions
  - `#[pyclass]` + `#[pymethods]` + `#[new]` for classes
  - `#[pymodule]` wrapper so the crate is importable after `pip install`
- [ ] `korvex generate --target python [--output <path>]`
  - Default output: `examples/python/`
- [ ] `.pyi` stub generation delegated to maturin (`maturin develop --strip`)

**Done when:** annotate a sample crate → `korvex generate --target python` → `maturin develop` →
`pip install` → function and class callable from Python with correct types.

---

## Milestone 3 — Node.js adapter (napi-rs)

> **Goal:** Generate a Rust bridge file that napi-rs can compile into a Node-importable package.

- [ ] `NapiBinding` reads IR and generates:
  - `#[napi]` on free functions
  - `#[napi]` + `#[napi(constructor)]` for classes
- [ ] `korvex generate --target node [--output <path>]`
  - Default output: `examples/node/`
- [ ] `.d.ts` generation delegated to napi-rs build step

**Done when:** annotate a sample crate → `korvex generate --target node` → `napi build` →
`npm install` → function and class callable from TypeScript with correct types.

---

## Milestone 4 — Validation + polish

> **Goal:** Production-quality error messages and a working quickstart.

- [ ] `korvex check`:
  - Validate all annotations are on supported item kinds
  - Every `class` has exactly one `new`
  - All types are in the supported mapping set
  - Exit non-zero on any error, with span-aware messages
- [ ] README quickstart: end-to-end example for both Python and Node.js

**Done when:** `korvex check` catches missing constructors and unsupported types with clear
messages.

---

## Post-MVP (deferred)

- WASI adapter (WIT + component model)
- `#[korvex::export(method)]` for non-constructor methods on classes
- `HashMap`, `Result`, and newtype support in the type mapping
- Macro syntax stabilization
- `0.1.0` release and first crates.io publish
