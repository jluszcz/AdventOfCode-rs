# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`aoc_util` is a **library-only** crate of shared utilities for solving [Advent of Code](https://adventofcode.com/)
puzzles in Rust: CLI setup, logging, input loading, and grid/math data structures.

There is no `[[bin]]` and no `src/main.rs`. Nothing here runs on its own — the crate is consumed by per-year puzzle
repos via a path dependency (`aoc_util = { path = "../AdventOfCode-rs" }`). Two consequences worth holding onto:

- **The CLI is never exercised by this repo's own tests.** `init()` calls `Args::parse()` (`src/lib.rs`), which only
  runs inside a consumer's binary. A change to the argument surface compiles and tests clean here while breaking
  every downstream puzzle binary.
- **Input paths are relative to the consumer's working directory**, not to this repo: `input/input` for real input
  and `input/example` for the example (`INPUT_PATH` / `TEST_INPUT_PATH` in `src/lib.rs`).

## Architecture

- **`src/lib.rs`** — the crate root: the `Input` enum (`Test` / `Actual`, with `FromStr`), the clap `Args` struct,
  `init()` (parse args → configure logging → load input) and `init_test()`, plus the input-file reader. Both return
  `Vec<String>`.
- **`src/grid/mod.rs`** — `Grid<T>` (a `Vec<Vec<T>>` newtype that `Deref`s to `[Vec<T>]` and indexes by `Point`),
  `Direction`, `Neighbor`, the `neighbors` / `neighbor_in_direction` helpers, and `print_grid`.
- **`src/math/mod.rs`** — `two_dimensional::Point<T = usize>` and `three_dimensional::Point<T = usize>` (with
  `distance` and `manhattan_distance`), `MinMax<T>` (built via `FromIterator`), `greatest_common_divisor`, and
  `least_common_multiple`. The `ToF64` trait bounds the floating-point distance methods.
- **`src/logging/mod.rs`** — `env_logger` setup. `init_test_logger()` is public; `init_logger()` is `pub(crate)` and
  reached through `init()`.

Log level is derived from the input type and `--verbose` rather than set directly (`src/lib.rs`): actual input
defaults to `Info`, example input to `Debug`, and `--verbose` moves each up one step. That is why an example run is
chattier than a real one without asking.

## Development Commands

- `cargo build` — build
- `cargo test` — run all tests
- `cargo fmt` / `cargo fmt --check` — format / check formatting
- `cargo clippy --all-targets -- -D warnings` — lint
- `pre-commit run --all-files` — the repo's commit gate (`.pre-commit-config.yaml`; includes `cargo fmt --check`)

`.github/workflows/ci.yml` is a thin caller of `jluszcz/github-utils/.github/workflows/rust-ci.yml@v1` — the steps
live in that shared workflow, not here. It runs build, test, `cargo fmt --check`, and
`cargo clippy --all-targets -- -D warnings`, so the commands above are the same set CI enforces.

## Conventions

- **`anyhow::Result` for every fallible API.** `anyhow::Context` is already a dependency; use it when an error would
  otherwise lose the detail that identifies the cause (a missing input file, an unparseable line).
- **Tests live inline** in `#[cfg(test)] mod tests` at the bottom of each module. There is no `tests/` directory and
  no integration test target.
- **`Point` is generic** — `Point<T = usize>`. The default keeps grid code terse, but signed instantiations are real
  and tested; don't narrow the parameter to `usize`.
- **The README is the public API reference.** It is the only API documentation this crate has, and consumers read it
  instead of the source. Any change to a public signature has to be reflected there in the same commit — the README
  has drifted from the code before, because nothing compiles its examples.
