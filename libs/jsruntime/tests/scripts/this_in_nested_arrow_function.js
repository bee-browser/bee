let a = {
  b: (() => {
    return () => this;
  })(),
};

print(a.b() === globalThis); ///=true
