#![crate_type = "proc-macro"]
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
    let s = Simple::new("hello".to_owned());
    println!("{}", s.field());
    //s.set_field("there".to_owned());
}
