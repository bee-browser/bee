print(typeof Object.assign); ///="function"
print(Object.assign.length); ///=2

const target = { a: 1, b: 2 };
const source = { b: 4, c: 5 };
const result = Object.assign(target, source);
print(result.a); ///=1
print(result.b); ///=4
print(result.c); ///=5
