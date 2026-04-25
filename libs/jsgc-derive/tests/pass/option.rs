use jsgc::Handle;
use jsgc_derive::Trace;

#[derive(Trace)]
struct A {
    handle: Handle<u8>,
}

#[derive(Trace)]
struct B {
    a: Option<A>,
}

fn main() {}
