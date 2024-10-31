print(true || true); ///=true
print(true || false); ///=true
print(false || true); ///=true
print(false || false); ///=false

print(0 || 0); ///=0
print(0 || 1); ///=1
print(1 || 0); ///=1

let a;

a = 0;
true || (a = 1);
print(a); ///=0

a = 0;
false || (a = 1);
print(a); ///=1
