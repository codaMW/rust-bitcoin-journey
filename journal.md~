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




## Day 2 — Ownership and Borrowing in Bitcoin-Style Transactions

### What I Did

- Reviewed Rust ownership and borrowing concepts with a focus on how they apply to Bitcoin software
- Implemented a simplified `Transaction` struct inspired by rust-bitcoin data models
- Wrote functions that explicitly:
  - Borrow a transaction immutably for inspection
  - Take ownership of a transaction and return it
- Used compiler errors as guidance to correct ownership mistakes

### Outcome
- Successfully compiled code that allows a transaction to be inspected before and after ownership transfer
- Gained practical experience passing structs by reference versus by value
- Confirmed that explicit ownership return is required to continue using consumed values

### Error Encountered
- Initial compiler error when attempting to use a transaction after it was moved
- Error message indicated use of a moved value due to ownership transfer
- The issue was resolved by returning ownership from the consuming function and re-binding the variable

### Key Takeaway
Ownership transfer in Rust is explicit and intentional. In Bitcoin-style software, borrowing is preferred for inspection and logging, while ownership transfer signals responsibility for a value’s lifecycle.

### Next Step
Continue exploring ownership errors deliberately to better understand how Rust enforces safety, and begin examining how ownership patterns appear in rust-bitcoin’s transaction validation code.

