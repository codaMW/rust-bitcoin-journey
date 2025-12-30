# Option and Result in Rust

This note documents the core concepts, patterns, and pitfalls of Rust’s
`Option` and `Result` enums, with examples relevant to Bitcoin and
Rust-based open source projects (e.g. Mostro, rust-bitcoin).

## 1. Option<T>

### Definition

```rust
enum Option<T> {
    Some(T),
    None,
}
