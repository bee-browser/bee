let a;

a = 0;
print(a ||= 1); ///=1
print(a); ///=1

a = 4;
print(a ||= 1); ///=4
print(a); ///=4

// short-circuit expressions using different types.

a = false;
print(a ||= undefined); ///=undefined
print(a); ///=undefined

a = false;
print(a ||= null); ///=null
print(a); ///=null

a = false;
print(a ||= 0); ///=0
print(a); ///=0
