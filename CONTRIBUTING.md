# Contributing to Korvex

Thank you for your interest in contributing. This document covers how to get started, the
development workflow, and the standards we hold contributions to.

## Prerequisites

- Rust stable (see `rust-toolchain.toml` for the pinned channel)
- `cargo` in your `PATH`

Optional but recommended:

- [`typos`](https://github.com/crate-ci/typos) — spell checker (`cargo install typos-cli`)
- [`cargo-clippy`](https://github.com/rust-lang/rust-clippy) — included with the toolchain

## Getting started

If `cargo` is not available, install the Rust toolchain first:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

The toolchain version is pinned in `rust-toolchain.toml` — `rustup` picks it up automatically.

```sh
git clone https://github.com/carlosferreyra/korvex
cd korvex
cargo xtask check
```

## Workspace layout

See [`crates/README.md`](crates/README.md) for the authoritative glossary of every crate and its
responsibility. Read it before making changes.

## Making changes

- Keep commits focused. One logical change per commit.
- Match the existing code style. Run `cargo fmt --all` to format, then verify with `cargo xtask check`.
- Do not reformat or refactor code unrelated to your change.
- No speculative abstractions. No features beyond what the issue asks for.

## Adding a new language adapter

See the **Adding a new adapter** section in [`AGENTS.md`](AGENTS.md).

## Checklist before opening a PR

```sh
cargo xtask test    # fmt + check + clippy + full test suite
```

Must pass before committing. Before pushing:

```sh
cargo xtask build   # includes test + release build + install
```

PRs that fail either of these will not be reviewed.

## Commit format

Commits MUST follow [Conventional Commits](https://www.conventionalcommits.org/).

Format: `<type>(<scope>): <description>`

Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`. Scope is the crate name
without the `korvex-` prefix (e.g. `feat(macros):`, `fix(core):`).

## Branch naming

Branches must follow `<agent>/<task>` format (e.g. `claude/define-roadmap`, `human/add-wasi`).
All branches target `main`.

## Pull request guidelines

- Title must be under 70 characters and follow conventional commit format.
- Use the body for detail. Link the relevant issue if one exists.
- Do not push to `main` directly. Open a PR immediately after your first push.
- Do not amend published commits.

## Versioning

All crates share a single version defined in `[workspace.package]` in the root `Cargo.toml`.
Pre-release versions follow `0.0.x`. The first crates.io release will be `0.1.0` and requires an
explicit maintainer decision — do not bump to it in a PR.

## License

By contributing, you agree that your contributions will be dual-licensed under [MIT](LICENSE-MIT)
and [Apache-2.0](LICENSE-APACHE), at the recipient's option.
