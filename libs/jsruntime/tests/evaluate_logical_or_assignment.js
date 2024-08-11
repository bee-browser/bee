let a;

a = 0;
print(a ||= 1); ///=1
print(a); ///=1

a = 4;
print(a ||= 1); ///=4
print(a); ///=4
