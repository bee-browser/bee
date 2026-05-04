class A {
  constructor(x) {
    this.x_ = x;
  }

  x() {
    return this.x_;
  }
}

print(typeof A); ///="function"
print(A.name); ///="A"

print(typeof A.prototype); ///="object"
print(A.prototype.constructor === A); ///=true
print(typeof A.prototype.x); ///="function"

const o = new A(1);
print(o.x()); ///=1
