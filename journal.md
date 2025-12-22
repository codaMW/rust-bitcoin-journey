## Day 1 — rust-bitcoin Environment Setup

### What I Did
- Cloned the rust-bitcoin repository
- Ran `cargo build --all-features`
- Generated documentation with `cargo doc --open`
- Ran the test suite with `cargo test`

### Outcome
- The full build succeeded on my Android userland Linux environment
- The build failed on Windows due to a dependency error related to `honggfuzz`

### Error Encountered
- `honggfuzz-rs does not currently support Windows`
- The failure occurred when building with `--all-features`

### Key Takeaway
Platform-specific tooling can affect builds when enabling all features. Windows builds may require WSL for full feature support.

### Next Step
Investigate which features pull in `honggfuzz` and how maintainers expect Windows users to build rust-bitcoin.

