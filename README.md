# `#[derive(accessors)]`: getters and setters for Rust (WIP)

**This is a work in progress!** We use the new macros 1.1 to implement
basic getters and setters.  This is useful if you have a library that
exports a struct with lots of fields, but you don't want to make the fields
themselves public.

```rust
#![feature(proc_macro)]

#[macro_use]
extern crate accessors;

#[derive(accessors)]
struct Simple {
    field: String,
}

impl Simple {
    pub fn new(field: String) -> Simple {
        Simple { field: field }
    }
}

fn main() {
    let mut s = Simple::new("hello".to_owned());

    // Automatically generated getter.
    println!("{}", s.field());

    // Automatically generated setter.
    s.set_field("there".to_owned());
}
```
