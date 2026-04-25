use jsgc_derive::Trace;

#[derive(Trace)]
struct A;

#[derive(Trace)]
struct B {}

fn main() {}
