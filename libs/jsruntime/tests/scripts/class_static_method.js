class A {
  static x() {
    return this;
  }
}

print(typeof A.x); ///="function"
print(A.x() === A); ///=true

const o = new A;
try {
  o.x();
} catch (e) {
  print(e.name); ///="TypeError"
}
