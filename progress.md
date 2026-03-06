# Bitcoin/Rust Development Log

## Goal
Become a fully funded open source Bitcoin/Rust contributor

## PRs Submitted
- PR #158: fix(take_order): correct amount label and help text for takebuy/takesell
  https://github.com/MostroP2P/mostro-cli/pull/158
- PR #159: fix(orders): prevent UUID truncation in listorders table
  https://github.com/MostroP2P/mostro-cli/pull/159

## Rust Concepts Encountered
- Ownership and move semantics (Action enum, non-Copy types)
- Borrowing with &reference to avoid moves
- match expressions with references (&action)
- cargo build vs cargo install --path .
- git workflow for forked repos

## Log
### 2026-03-06
- Fixed takebuy/takesell amount label and help text (PR #158)
- Fixed UUID truncation in listorders table (PR #159)
- Diagnosed trade index sync issue (not a code bug)
- Learned: non-Copy enums must be matched by reference to avoid move errors


Add today's entry:
```
### 2026-03-07
- Learned Result<T, E> - Ok and Err variants
- Practiced ? operator for error propagation
- Chained multiple fallible functions
- Key insight: ? stops the function immediately and returns Err to caller
- Next: anyhow crate, then back to mostro-cli contributions
