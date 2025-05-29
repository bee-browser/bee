let a = {
  b: () => this,
};

print(a.b() === globalThis); ///=true
