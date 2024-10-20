print(true && true); ///=true
print(true && false); ///=false
print(false && true); ///=false
print(false && false); ///=false

print(0 && 1); ///=0
print(1 && 0); ///=0
print(1 && 2); ///=2

let a;

a = 0;
true && (a = 1);
print(a); ///=1

a = 0;
false && (a = 1);
print(a); ///=0
