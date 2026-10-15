class B extends class A {
  a = 1;
} {
  b = 2;
}

print(B.name); ///="B"

const o = new B;
print(o instanceof B); ///=true
try {
  o instanceof A;
} catch (e) {
  print(e instanceof ReferenceError); ///=true
}
print(o.a); ///=1
// TODO(fix): print(o.b); ///=2
