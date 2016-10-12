#![feature(proc_macro)]

#[macro_use]
extern crate accessors;

#[derive(getters, setters)]
#[setters(into)]
struct Simple {
    field: String,
}

fn main() {
    let mut s = Simple { field: "hello".to_owned() };
    println!("{}", s.field());
    s.set_field("there");
}
