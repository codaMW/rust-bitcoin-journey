01 — Ownership and Borrowing in Bitcoin-Style Transactions

## Context

Rust’s ownership model is fundamental to how safety is enforced in
Bitcoin-related software. In projects like `rust-bitcoin`, ownership
is used deliberately to prevent invalid states, accidental mutation,
and unnecessary data duplication.

This exercise introduces ownership and borrowing using a simplified
Bitcoin-style transaction model. The goal is not to simulate Bitcoin
perfectly, but to practice Rust concepts **as they appear in real
open-source Bitcoin codebases**.


## Learning Objectives

By completing this exercise, I aim to:

- Understand the difference between *owning* and *borrowing* data
- Learn when ownership transfer is appropriate in system-level code
- Practice passing structs by reference without cloning
- Observe how Rust enforces correct data usage at compile time


## Problem Description

I have been given a simplified `Transaction` struct representing a Bitcoin
transaction.

I must implement two functions:

1. A function that **takes ownership** of a transaction
2. A function that **borrows** a transaction immutably

Both functions should operate on the same transaction instance in a
safe and explicit way.


## Constraints

- Do **not** use `.clone()`
- Do **not** use global variables
- Do **not** use smart pointers (`Rc`, `Arc`)
- Do **not** introduce lifetimes manually
- Prefer explicit ownership and borrowing
- Code must compile successfully


## Why This Matters for Bitcoin Software

In Bitcoin systems:

- Transactions are large and expensive to clone
- Ownership transfer signals responsibility and lifecycle control
- Borrowing enables inspection without restricting future use
- Immutability is preferred wherever possible

This exercise mirrors how Bitcoin software distinguishes between:
- *Inspecting* data (borrowing)
- *Consuming* data (ownership transfer)



## Success Criteria

This exercise is complete when:

- The transaction can be inspected before and after ownership transfer
- The compiler enforces correct usage without runtime checks
- Ownership decisions are intentional and documented
- No unnecessary cloning is used


