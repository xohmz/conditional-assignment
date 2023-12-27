# conditional-assignment

![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/xohmz/conditional-assignment/rust.yml?branch=main)
![Crates.io](https://img.shields.io/crates/v/conditional-assignment)
![Crates.io (latest)](https://img.shields.io/crates/dv/conditional-assignment)

This is a very simple, small crate to help make conditional assignments more ergonomic.

## Intent

The goal is to make the below look a little better.

```rust
let condition = 0 < 1;
let outcome = if condition {
    "true"
} else {
    "false"
};
```

## Examples

### Basic

#### Eager

```rust
use conditional_assignment::Pick;

let condition = 0 < 1;
let outcome = condition.pick("true", "false");
```

#### Lazy

```rust
use conditional_assignment::Pick;

let condition = 0 < 1;
let outcome = condition.pick_lazy(
    || {
        // This won't be evaluated and
        // panic when condition is false.
        assert!(condition);
        "true"
    },
    || {
        // This won't be evaluated and
        // panic when condition is true.
        assert!(!condition);
        "false"
    },
);
```

## Minimum Supported Rust Version (MSRV)

According to [cargo-msrv](https://github.com/foresterre/cargo-msrv), the MSRV
is `1.56.1`. Given how simple this crate is, I wouldn't be surprised if the
MSRV is lower, but I didn't check. Try it!

### License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
