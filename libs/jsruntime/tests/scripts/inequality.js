print(undefined != undefined); ///=false
print(undefined != null); ///=false
print(true != true); ///=false
print(false != false); ///=false
print(0 != 0); ///=false
print(+0 != -0); ///=false
print(1 != 1); ///=false
print(Infinity != Infinity); ///=false
print('a' != 'a'); ///=false

print(undefined != true); ///=true
print(null != true); ///=true
print(true != false); ///=true
print(0 != 1); ///=true
print(0 != Infinity); ///=true
print(0 != NaN); ///=true
print(1 != Infinity); ///=true
print(1 != NaN); ///=true
print(Infinity != NaN); ///=true
print(NaN != NaN); ///=true
print('a' != 'b'); ///=true

let a, b;

a = true;
b = true;
print(a != true); ///=false
print(true != a); ///=false
print(a != a); ///=false
print(a != b); ///=false
print(b != a); ///=false

a = true;
b = false;
print(a != false); ///=true
print(false != a); ///=true
print(a != b); ///=true
print(b != a); ///=true
