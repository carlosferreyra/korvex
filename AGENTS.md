# Korvex — Agent & Contributor Guide

This file is the authoritative guide for AI agents and human contributors working on this
repository. All agents must read this file before taking any action.

## Project overview

Korvex generates idiomatic language bindings from annotated Rust code. Annotate with
`#[korvex::export]`, select a target language via Cargo features, and Korvex emits the binding glue.
No `.udl` files, no separate schema — the source code is the contract.

It is a composable alternative to [UniFFI](https://github.com/mozilla/uniffi-rs). Each language
backend is an independent adapter crate; the core engine is decoupled from all of them.

## Workspace layout

```text
crates/
  korvex/          Public SDK — the crate downstream consumers depend on
  korvex-cli/      Binary: inspect / check / generate commands
  korvex-adapter/  BindingAdapter trait — the contract all language adapters implement
  korvex-core/     Binding engine — IR processing and adapter dispatch
  korvex-macros/   Proc-macro crate — #[korvex::export] and related attributes
  korvex-types/    Shared IR type definitions (no deps — foundation of the graph)
  korvex-node/     Node.js adapter (napi-rs)
  korvex-python/   Python adapter (pyo3)
  korvex-wasi/     WASI adapter (WIT + wasm component model)
  xtask/           Dev automation (test, debug, install) — not published
```

Dependency order (nothing may introduce a cycle):

```text
korvex-types   → korvex-macros
korvex-types   → korvex-adapter
korvex-adapter → korvex-python / korvex-node / korvex-wasi
korvex-adapter → korvex-core
korvex-core    → (dispatches to adapters at runtime)
korvex         → korvex-types + korvex-core + korvex-macros + korvex-adapter
korvex-cli     → korvex
```

## Versioning

- Pre-release versions follow `0.0.x`. Do not bump to `0.1.0` — that is the first crates.io release
  and requires an explicit decision.
- All crates share a single version via `[workspace.package]` in the root `Cargo.toml`.

## Code rules

- Edition: **2024** (`edition = "2024"` in `[workspace.package]`).
- No crate other than `korvex-types` may define its own IR types.
- `korvex-macros` must remain `proc-macro = true` and have minimal dependencies.
- Adapter crates (`korvex-python`, `korvex-node`, `korvex-wasi`) must only depend on
  `korvex-adapter` — never on each other and never on the `korvex` SDK crate.
- Match existing code style. Do not reformat unrelated code.
- No speculative abstractions. No features beyond what is asked.

## Adding a new adapter

1. Create `crates/korvex-<lang>/` with `Cargo.toml` and `src/lib.rs`.
2. Depend only on `korvex-adapter`.
3. Add the new crate to `[workspace.members]` in the root `Cargo.toml`.
4. Document it in `crates/README.md` and the crate table in `README.md`.
5. Do not add it as a dependency of `korvex` or `korvex-cli` unless explicitly asked.

## CLI commands (intended shape)

```sh
korvex inspect   # print the IR derived from annotated types in the current crate
korvex check     # validate annotations; exit non-zero on errors
korvex generate  # write binding files to disk for use by maturin, wasm-pack, etc.
```

## Commit and PR hygiene

Commits MUST follow [Conventional Commits](https://www.conventionalcommits.org/). This is not
optional — non-conforming commits will be rejected.

Format: `<type>(<scope>): <description>`

Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`. Scope is the crate name
without the `korvex-` prefix (e.g. `feat(macros):`, `fix(core):`).

Examples:

```text
feat(macros): add #[korvex::export] attribute
fix(core): handle empty IR when no types are annotated
docs: add cargo install instructions to README
chore(deps): bump pyo3 to 0.23
```

- Keep commits focused. One logical change per commit.
- PR titles must be short (under 70 characters) and also follow conventional commit format.
- Branch names must follow `<agent>/<task>` format (e.g. `claude/define-roadmap`,
  `copilot/fix-core`, `human/add-wasi`). All branches target `main` (or `dev` if it exists).
- Do not push to `main` directly. Always open a **draft PR** immediately after the first push. All
  PRs start as drafts. A CI workflow will auto-promote drafts to ready when all checks pass. Do not
  manually mark a PR ready — let CI do it.
- Do not amend published commits.

## Environment bootstrap

If `cargo` is not available (e.g. ephemeral CI or claude.ai/code sessions):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

The toolchain version and edition are pinned in the repo — do not override them:

- `rust-toolchain.toml` — pins the exact toolchain channel (`rustup` reads this automatically)
- `rustfmt.toml` — sets edition for formatting (`cargo fmt` reads this automatically)

After that, all dev tasks run through xtask:

```sh
cargo xtask check            # fmt --check + cargo check + clippy -D warnings
cargo xtask test             # check + unit tests + integration tests
cargo xtask test <filter>    # same but scoped to a test filter
cargo xtask build            # test + release build + install CLI locally
cargo xtask build --force    # same, force reinstall
```

These commands form a pyramid — each includes the previous checkpoint. Use them as gates during a
session and in CI:

| Checkpoint          | When                        |
| ------------------- | --------------------------- |
| `cargo xtask check` | after every meaningful edit |
| `cargo xtask test`  | before committing           |
| `cargo xtask build` | before pushing              |

## Checking your work

```sh
cargo xtask test    # must pass before marking any task complete
```
