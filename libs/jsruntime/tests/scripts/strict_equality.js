print(undefined === undefined); ///=true
print(null === null); ///=true
print(true === true); ///=true
print(false === false); ///=true
print(0 === 0); ///=true
print(+0 === -0); ///=true
print(1 === 1); ///=true
print(Infinity === Infinity); ///=true
print('a' === 'a') ///=true

print(undefined === null); ///=false
print(undefined === true); ///=false
print(undefined === false); ///=false
print(undefined === 0); ///=false
print(undefined === 1); ///=false
print(undefined === Infinity); ///=false
print(undefined === NaN); ///=false
print(undefined === 'a'); ///=false
print(null === true); ///=false
print(null === false); ///=false
print(null === 0); ///=false
print(null === 1); ///=false
print(null === Infinity); ///=false
print(null === NaN); ///=false
print(null === 'a'); ///=false
print(true === false); ///=false
print(true === 0); ///=false
print(true === 1); ///=false
print(true === Infinity); ///=false
print(true === NaN); ///=false
print(true === 'a'); ///=false
print(false === 0); ///=false
print(false === 1); ///=false
print(false === Infinity); ///=false
print(false === NaN); ///=false
print(false === 'a'); ///=false
print(0 === 1); ///=false
print(0 === Infinity); ///=false
print(0 === NaN); ///=false
print(1 === Infinity); ///=false
print(1 === NaN); ///=false
print(Infinity === NaN); ///=false
print(NaN === NaN); ///=false
print(0 === 'a'); ///=false
print(Infinity === 'a'); ///=false
print(NaN === 'a'); ///=false
print('a' === 'b') ///=false

let a, b;

a = undefined;
b = undefined;
print(a === undefined); ///=true
print(undefined === a); ///=true
print(a === b); ///=true
print(b === a); ///=true

a = undefined;
b = null;
print(a === null); ///=false
print(null === a); ///=false
print(a === b); ///=false
print(b === a); ///=false
