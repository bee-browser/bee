let a;

a = null;
print(a ??= 1); ///=1
print(a); ///=1

a = 0;
print(a ??= 1); ///=0
print(a); ///=0
