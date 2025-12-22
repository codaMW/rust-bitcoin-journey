# Building rust-bitcoin on Different Platforms

## Observed Issue
Building `rust-bitcoin` with `cargo build --all-features` fails on native Windows.

## Root Cause
- The `honggfuzz` crate is included when enabling all features
- `honggfuzz` is a fuzzing tool
- It does not support native Windows builds
- Maintainers recommend using WSL for full feature builds on Windows

## Key Insight
Not all features are meant for end users. Some exist for:
- Fuzz testing
- CI environments
- Maintainer-only workflows

## Practical Guidance
- Use `cargo build` (without `--all-features`) for day-to-day development
- Use WSL if building all features on Windows
- Treat feature flags as part of the project’s tooling strategy

