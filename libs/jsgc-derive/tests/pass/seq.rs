use jsgc::Handle;
use jsgc::Seq;
use jsgc_derive::Trace;

#[derive(Trace)]
struct A {
    handle: Handle<u8>,
}

#[derive(Trace)]
struct B<'a> {
    a: [A; 4],
    b: &'a [A],
    c: Vec<A>,
    d: Seq<A>,
}

fn main() {}
