let a = {
  b: () => this,
};

print(a.b() === globalThis); ///=true

const o = {};
print(a.b.call(o) === globalThis); ///=true
