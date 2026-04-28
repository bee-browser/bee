print(typeof Object.create); ///="function"
print(Object.create.length); ///=2

const o = Object.create({}, { a: 1 });
print(o.a); ///=1
