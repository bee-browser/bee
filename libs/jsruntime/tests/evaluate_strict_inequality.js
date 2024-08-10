print(undefined !== undefined); ///=false
print(null !== null); ///=false
print(true !== true); ///=false
print(false !== false); ///=false
print(0 !== 0); ///=false
print(+0 !== -0); ///=false
print(1 !== 1); ///=false
print(Infinity !== Infinity); ///=false

print(undefined !== null); ///=true
print(undefined !== true); ///=true
print(undefined !== false); ///=true
print(undefined !== 0); ///=true
print(undefined !== 1); ///=true
print(undefined !== Infinity); ///=true
print(undefined !== NaN); ///=true
print(null !== true); ///=true
print(null !== false); ///=true
print(null !== 0); ///=true
print(null !== 1); ///=true
print(null !== Infinity); ///=true
print(null !== NaN); ///=true
print(true !== false); ///=true
print(true !== 0); ///=true
print(true !== 1); ///=true
print(true !== Infinity); ///=true
print(true !== NaN); ///=true
print(false !== 0); ///=true
print(false !== 1); ///=true
print(false !== Infinity); ///=true
print(false !== NaN); ///=true
print(0 !== 1); ///=true
print(0 !== Infinity); ///=true
print(0 !== NaN); ///=true
print(1 !== Infinity); ///=true
print(1 !== NaN); ///=true
print(Infinity !== NaN); ///=true
print(NaN !== NaN); ///=true

let a, b;

a = undefined;
b = undefined;
print(a !== undefined); ///=false
print(undefined !== a); ///=false
print(a !== b); ///=false
print(b !== a); ///=false

a = undefined;
b = null;
print(a !== null); ///=true
print(null !== a); ///=true
print(a !== b); ///=true
print(b !== a); ///=true
