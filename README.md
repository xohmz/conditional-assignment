# conditional-assignment

![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/xohmz/conditional-assignment/rust.yml?branch=main)
![Crates.io](https://img.shields.io/crates/v/conditional-assignment)
![Crates.io (latest)](https://img.shields.io/crates/dv/conditional-assignment)

This is a very simple, small crate to help make conditional assignments more
ergonomic.

## Minimum Supported Rust Version (MSRV)

According to [cargo-msrv](https://github.com/foresterre/cargo-msrv), the MSRV
is `1.56.1`. Given how simple this crate is, I wouldn't be surprised if the
MSRV is lower, but I didn't check. Try it!

## Why

Sometimes this:

```rust,ignore
let outcome = condition.pick_lazy(positive, negative)
```

looks better than this:

```rust,ignore
let outcome = if condition {
    positive
} else {
    negative
}
```

## Examples

Below are a few examples. See [tests](./tests/mod.rs) for more. These are just
examples to show the concept. If you find yourself nesting lots of `pick`s,
there is probably a better/prettier way. Chaining `pick`s seems better, but
still within reason. In some situations `map` makes more sense.

```rust
// Bring the trait into scope so you can use it.
use conditional_assignment::ConditionalAssignment;

// Return a response string depending on whether
// the bool is true or false.
fn get_response_string(correct: bool) -> String {
    correct.pick(
        "That's correct, well done!",
        "Not quite, try again!"
    ).to_owned()
}

// Determine if a number is a multiple of 2, 3, and 5.
fn multiple_of_2_3_5(num: u32) -> bool {
    (num % 2 == 0).pick(
        (num % 3 == 0).pick(
            (num % 5 == 0).pick(
                true,
                false
            ),
            false
        ),
        false,
    )
}

// Get whether a &str is lowercase and get its length. 
// If the &str is none, returns (None, None).
// If the &str is empty, return (None, Some(0)).
// If the &str is not empty, return (Some(<is lowercase>), Some(<length>)).
// ...
// Except not really! `a.unwrap()` can panic, because it gets evaluated before
// we know that the `Option<&str>` is `Some(&str)`
fn is_all_lower_and_len(a: Option<&str>) -> (Option<bool>, Option<usize>) {
    a.pick(
        {
            let a = a.unwrap(); // This can panic! :(
            let len_a = a.len();

            (
                (len_a == 0).pick(None, Some(a == a.to_lowercase())),
                Some(len_a),
            )
        },
        (None, None),
    )
}
```

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
