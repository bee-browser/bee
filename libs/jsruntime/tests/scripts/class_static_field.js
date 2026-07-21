class A {
  static A;
  static B = 1;
  static C = this.B + 1;
}

print(A.A); ///=undefined
print(A.B); ///=1
print(A.C); ///=2
