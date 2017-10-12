# Demo Rust Cargo TDD

Rust is a great language for systems programming.

Cargo is the Rust package manager.

TDD is test driven development.

This demo shows how to use Rust and Cargo for TDD, starting with zero knowledge of Rust.

For the official documenation see https://www.rust-lang.org/


## Install Rust and Cargo

Run:

```shelll
$ curl https://sh.rustup.rs -sSf | sh
```

Verify:

```
$ rustc --version
rustc 1.20.0

$ cargo --version
cargo 0.21.0


## Create a project

New:

```shell
$ cargo new demo && cd demo
Created library `demo` project
```

Build:

```shell
$ cargo build
Compiling demo v0.1.0 (file:...)
Finished dev [unoptimized + debuginfo] target(s) in 0.21 secs
```

Rust and Cargo automatically created some placeholder files, such as a project configuration file `Cargo.toml` and an example library source code file `lib.rs`.


## Sample test

See the sample file `src/lib.rs` that defines a module and a test:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

Explanation:

```rush
#[cfg(test)]           // ??
mod tests {            // Define a module named "tests"
    #[test]            // Annotation that states the next function is a test
    fn it_works() {    // Define a function as usual
    }
}
```


Test:

```shell
$ cargo test
  Compiling demo v0.1.0 (file:...)
   Finished dev [unoptimized + debuginfo] target(s) in 0.52 secs
    Running ~/demo/target/debug/deps/demo-e0dde10b16c02043

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests demo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```


## Write a test function and normal function

```rust
#[cfg(test)]           // ??
mod tests {            // Define a module named "tests"
    use super::*;      // Bring in code from the outer module

    #[test]            // Annotation that states the next function is a test
    fn foo_test() {    // Define a function as usual
      assert!(foo());  // Assert is a test macro that passes or panics
    }

}

pub fn foo() -> bool { // Define a public function named "foo" that returns a boolean 
    true               // Return true
}
```

Details:

  * The line `#[test]` is an annotation before the line `fn`. This annotation indicates this is a test function, so that the test runner knows to treat this function as a test. We could also use non-test functions in the tests module, such as functions to help set up common scenarios, or perform common operations.

  * The line `use super::*;` is necessary because the tests module is a regular module that follows the usual visibility rules. The test code is in an inner module, so we need to bring the code under test in the outer module into the scope of the inner module. We use a glob so that anything we define in the outer module is available to this tests module.

  * The `pub` keyword means public. If this function were not public, then the test would show `warning: function is never used: foo` and `note: #[warn(dead_code)] on by default`.

Test:

```shell
$ cargo test
   Compiling demo v0.1.0 (file:...)
    Finished dev [unoptimized + debuginfo] target(s) in 0.99 secs
     Running target/debug/deps/demo-e0dde10b16c02043

running 1 test
test tests::foo_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests demo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Details:

  * The most important result is `test result: ok`, which proves the test succeeded.

