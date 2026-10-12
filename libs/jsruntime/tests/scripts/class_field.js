class A {
  a;
  b = 1;
  c = this.b + 1;
}

const o = new A;
print(o.a); ///=undefined
print(o.b); ///=1
print(o.c); ///=2
