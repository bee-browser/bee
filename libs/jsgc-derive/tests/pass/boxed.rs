use jsgc::Handle;
use jsgc_derive::Trace;

#[derive(Trace)]
struct A {
    handle: Handle<u8>,
}

#[derive(Trace)]
struct B {
    a: Box<A>,
}

fn main() {}
