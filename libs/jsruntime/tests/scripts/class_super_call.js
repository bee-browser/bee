class A {
  constructor() {
    print('A'); ///#1="A"
    this.a_ = 'a';
  }

  a() {
    return this.a_;
  }
}

class B extends A {
  b() {
    return 'b';
  }
}

class C extends B {
  constructor() {
    print('C'); ///#0="C"
    super();
    this.c_ = 'c';
  }

  c() {
    return this.c_;
  }
}

const o = new C;
print(o.a()); ///#2="a"
print(o.b()); ///#3="b"
print(o.c()); ///#4="c"
