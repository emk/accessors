# `#[derive(accessors)]`: getters and setters for Rust (WIP)

**This is a work in progress!** We use the new macros 1.1 to implement
getters, and very soon setters.  This is intended to be used with some very
field-heavy structs where we want to hide the implementation of getters and
setters for future compatibility.

See `examples/simple.rs` for an example of how this works.
