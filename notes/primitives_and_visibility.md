# Rust Bitcoin Primitives: Visibility, Construction, and Safety

## Why the primitives module exists

The `primitives` crate in rust-bitcoin defines the fundamental Bitcoin data types
(blocks, transactions, scripts, hashes, amounts) in their most minimal and
invariant-preserving form.

These types are:
- Shared across the entire rust-bitcoin ecosystem
- Designed to prevent invalid Bitcoin states at compile time
- Carefully constructed to control how values enter the system

Primitives are not application-level conveniences.
They are correctness boundaries.


## Visibility as a safety tool

One of the most noticeable design choices in rust-bitcoin primitives is that
many struct fields are **private**, even when the struct itself is public.

This enforces:
- Controlled construction
- Validation at creation time
- Invariant preservation

Rust prevents external code from directly constructing or mutating critical
types unless explicitly allowed.

This mirrors Bitcoin Core’s philosophy:
> “Make invalid states unrepresentable.”


## Constructor-based APIs

Instead of exposing fields, primitives expose constructors and methods.

Example pattern:

- Struct fields: private
- `new(...)` or `from_*` functions: public
- Access through methods, not direct field reads

This ensures:
- All instances are created intentionally
- Future validation can be added without breaking APIs
- External code cannot bypass invariants

I encountered this pattern firsthand when my `Address` struct failed to compile
until I introduced a constructor.


## Enums model protocol variants

Bitcoin protocol variants (address types, script types, hash types) are commonly
modeled as enums.

This provides:
- Exhaustive pattern matching
- Compiler-enforced correctness
- Clear domain modeling

Example:
- P2PKH
- P2SH
- P2WPKH
- P2TR

Rust forces me to handle all cases explicitly, which aligns with Bitcoin’s
security requirements.


## Key takeaway

Rust’s visibility rules are not an inconvenience.
They are a core part of how rust-bitcoin enforces safety, clarity, and correctness.

Understanding *why* code is structured this way is more important than writing
code that merely compiles.

