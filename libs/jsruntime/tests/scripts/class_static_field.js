class A {
  static A;
  static B = 1;
  static C = this.B;
}

print(A.A); ///=undefined
print(A.B); ///=1
// TODO: print(A.C); ///=1
