# rustricks

## Overview

This repo contains many specific cases of some of rust special features. Each case is presented in an 'example' (each folder in the `examples/` directory corresponds to a specfic feature that we deal with in details).

For each feature (let's call it `feats`), you can :

- build the example using `cargo build --example feats`

- build & run using `cargo run --example feats` 

`cargo build` and `cargo run` will compile and run the file in `src/bin/main.src`.

## Examples

### a. Result type & error handling (`result`)

- Panic attack
- Recoverable & Unrecoverable errors
- Result type
- `.unwrap()` method

### b. Lifetime (`lifetime`)

- Reference lifetime in Rust
- Ownership
- Borrowing

### c. Threads and Concurrency (`threads`)

- Creating & running threads
- Message-passing between threads
- Shared-state concurrency