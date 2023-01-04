# Result type and Error handling in Rust

When it comes to errors in Rust language, there are two types:

- **Unrecoverable** errors: It is just the error that stop your program from running. Sometime we refer to them as "runtime errors". For instance, when the programmer try to access beyond the bounds of an array or to some ressource or file that is not (or no more) available.
- **Recoverable** errors: the ones that you can catch while your program is running. No runtime errors, and a little similar to "throw-catch" procedures in higher level languages. Once the error is catched, the programmer can decide what's going be done depending on the situation "and recover from it".

## Unrecoverable errors: Panic attack

***What happen when errors are unrecoverable? What is the `panic!` macro?***

Panic macro is what Rust use to stop a program when some unallowed actions are performed using runtime. In other words, it means that an unmanaged runtime error has been triggered and the executable will either perform:

- stack unwinding which is the default behavior and means that it will *slowly cleaning all function calls in the stacks*
- or abort everything and let the operating system handling the stack. This can be used by specifying this in the `Cargo.toml` file:

    ```toml
    [profile.release]
    panic=abort
    ```

Now, let's consider this example. We will use the panic macro explicitly in our code in order to "simulate" the case when some unrecoverable error is triggered. Actually, we are basically forcing the program to "panic".

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

**Result type** is what Rust implements to replace exceptions.

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
The [`match` keyword](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) allows pattern matching and it can be used like a C's `switch-case` keywords.
In this example, there are two possibiloties after opening the file. Either no error is raised and `f` will be a reference to the file, or an error will be raised and the program will panic (cause a runtime error).
If the file does not exist, the program will panic and print "`problem with the file7` +  `<the error message>`". The output we get will look like this:

```plain
> thread 'main' panicked at 'Problem with the file Os { code: 2, kind: NotFound, message: "No such file or directory" }', examples/result/main.rs:7:23
```

## Sources

- The Dev Method (2022), *Rust: Error Handling*, from <https://youtu.be/y3wUCb-uS3g>
- *The Rust Programming Language Book* (2022), from <https://doc.rust-lang.org/stable/book/>
- *Rust By Example*, from https://doc.rust-lang.org/stable/rust-by-example/
- Google (2022), *Comprehensive Rust course*, from <https://google.github.io/comprehensive-rust/>