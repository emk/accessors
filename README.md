# `#[derive(accessors)]`: getters and setters for Rust (WIP)

**This is a work in progress!** We use the new [macros 1.1][] support in
nightly Rust to automatically generate basic getters and setters.  This is
useful if you have a library that exports a struct with lots of fields, but
you don't want to make the fields themselves public.

```rust
#![feature(proc_macro)]

#[macro_use]
extern crate accessors;

#[derive(getters, setters)]
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
    println!("{}", s.field());
    s.set_field("there".to_owned());
}
```

[macros 1.1]: https://users.rust-lang.org/t/macros-and-syntax-extensions-and-compiler-plugins-where-are-we-at/7600
