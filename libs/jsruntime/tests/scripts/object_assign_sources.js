const o1 = { a: 1, b: 1, c: 1 };
const o2 = { b: 2, c: 2 };
const o3 = { c: 3 };

const obj = Object.assign({}, o1, o2, o3);

print(obj.a); ///=1
print(obj.b); ///=2
print(obj.c); ///=3
