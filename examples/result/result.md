# Result type and Error handling in Rust


## Panic attack (`panic!` macro)
***or what happen when errors are unrecoverable***

Panic macro means that an unmanaged runtime error has been triggered and the executable will either perform:
- stack unwinding which is the default behavior and means that it will *slowly cleaning all function calls in the stacks*
- or abort everything and let the operating system handling the stack. This can be used by specifying this in the `Cargo.toml` file:
    ```toml
    [profile.release]
    panic=abort
    ```

Now, let's consider this example.

```rust
fn panic_attack(){
    panic!("some message");
}

fn main(){
    panic_attack();
}
```

To enable backtrace, just run `RUST_BACKTRACE=1` with `cargo run` command.
```plain
➜  rustricks git:(main) ✗ RUST_BACKTRACE=1 cargo run --example result 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/result`
thread 'main' panicked at 'some message', examples/result/main.rs:2:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/panicking.rs:65:14
   2: result::panic_attack
             at ./examples/result/main.rs:2:5
   3: result::main
             at ./examples/result/main.rs:6:5
   4: core::ops::function::FnOnce::call_once
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/ops/function.rs:251:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

This behavior is the same that a user will get when having a runtime error. Let's consider now this example, in which the user tries to access an element of a vector (growable array) through an invalid index:

```rust
fn main(){
    let vec = vec![1, 2, 3, 4, 5];
    println!("Access to an inaccessible item {?:}", vec[10]);
}
```

## Recoverable errors

```rust
enum Result<T, E>{
    Ok(T),
    Err(R),
}
```

Let's consider a code example in which we read a text file:

```rust
use std::fs::File; // standard library => File system => File object 

fn main(){
    let f = File::open("./examples/result/textfile.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem with the file {:?}", error),
    };
}

```


## Sources

- The Dev Method (2022), *Rust: Error Handling*, from <https://youtu.be/y3wUCb-uS3g>
- *The Rust Programming Language Book* (2022), from <https://doc.rust-lang.org/stable/book/>
- Google (2022), *Comprehensive Rust course*, from <https://google.github.io/comprehensive-rust/>