class A {
  a() {
    return 1;
  }
}

class B extends A {
  b() {
    return 2;
  }
}

const o = new B;
// TODO(feat): print(o instanceof A); ///=true
// TODO(feat): print(o instanceof B); ///=true
print(o.a()); ///=1
print(o.b()); ///=2
