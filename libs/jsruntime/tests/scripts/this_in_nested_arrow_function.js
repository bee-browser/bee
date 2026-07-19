let a = {
  b: (() => {
    return () => this;
  })(),
};

print(a.b() === globalThis); ///=true
print(a.b.call(null) === globalThis); ///=true

const o = {};
print(a.b.call(o) === globalThis); ///=true
