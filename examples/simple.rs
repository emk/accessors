#![feature(proc_macro)]

#[macro_use]
extern crate accessors;

#[derive(getters, setters)]
#[setters(into = true)]
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
    s.set_field("there");
}
