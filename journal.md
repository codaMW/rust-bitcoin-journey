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


## Day 3 — Bitcoin Primitives & Rust Visibility

### What I Did
- Studied the `primitives` module in rust-bitcoin
- Modeled Bitcoin address variants using Rust enums
- Combined enums and structs to represent an Address type
- Encountered and resolved Rust visibility errors using constructor-based design

### Outcome
- Successfully implemented a Bitcoin-style `Address` struct with private fields
- Learned how Rust enforces invariant-safe construction across modules
- Gained hands-on experience with patterns used in rust-bitcoin primitives

### Error Encountered
- Compiler error when constructing a struct with private fields from another module
- Error code: `E0451` (attempted access to private struct fields)

### Key Takeaway
Rust’s privacy model is a deliberate safety mechanism.
Bitcoin primitives should not expose their internal state freely.
Constructor-based APIs are essential for correctness and maintainability.

### Reflection
This was my first time experiencing Rust actively preventing me from designing an unsafe API.
The solution felt aligned with how Bitcoin Core and rust-bitcoin enforce correctness.

### Next Step
- Continue reading rust-bitcoin primitives with a focus on enums and type modeling
- Practice implementing accessor methods and match-based logic


## Day 4 — Establishing a Baseline

Today I made a deliberate decision to preserve my original Bitcoin
transaction implementation as a baseline.

Instead of immediately refactoring or optimizing, I committed the
initial model to create a clear reference point for future improvements.

Key lesson:
Good engineering is iterative. Preserving working code allows me to
reason about trade-offs, track progress, and understand *why* changes
are introduced rather than blindly rewriting.

This mirrors how Bitcoin and other critical open source systems evolve.

