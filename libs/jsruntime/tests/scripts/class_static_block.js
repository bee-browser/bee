class A {
  static A = 1;
  static {
    this.B = this.A + 1;
  }
}

print(A.A); ///=1
print(A.B); ///=2
