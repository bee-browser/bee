print(undefined == undefined); ///=true
print(undefined == null); ///=true
print(true == true); ///=true
print(false == false); ///=true
print(0 == 0); ///=true
print(+0 == -0); ///=true
print(1 == 1); ///=true
print(Infinity == Infinity); ///=true

print(undefined == true); ///=false
print(null == true); ///=false
print(true == false); ///=false
print(0 == 1); ///=false
print(0 == Infinity); ///=false
print(0 == NaN); ///=false
print(1 == Infinity); ///=false
print(1 == NaN); ///=false
print(Infinity == NaN); ///=false
print(NaN == NaN); ///=false

let a, b;

a = true;
b = true;
print(a == true); ///=true
print(true == a); ///=true
print(a == a); ///=true
print(a == b); ///=true
print(b == a); ///=true

a = true;
b = false;
print(a == false); ///=false
print(false == a); ///=false
print(a == b); ///=false
print(b == a); ///=false
