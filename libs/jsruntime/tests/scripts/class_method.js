class A {
  a() {
    return this === o;
  }
}

print(typeof A.prototype.a); ///="function"
print(A.prototype.a.name); ///="a"

const o = new A;
print(o.a()); ///=true
