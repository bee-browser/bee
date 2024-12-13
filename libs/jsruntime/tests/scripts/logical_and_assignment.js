let a;

a = 0;
print(a &&= 1); ///=0
print(a); ///=0

a = 4;
print(a &&= 1); ///=1
print(a); ///=1

// short-circuit expressions using different types.

a = true;
print(a &&= undefined); ///=undefined
print(a); ///=undefined

a = true;
print(a &&= null); ///=null
print(a); ///=null

a = true;
print(a &&= 0); ///=0
print(a); ///=0
