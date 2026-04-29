const o = Object.defineProperties({}, {
  a: {
    value: 1,
  },
  b: {
    value: 2,
    configurable: true,
  },
  c: {
    value: 3,
    configurable: false,
  },
});

print(o.a); ///=1
print(o.b); ///=2
print(o.c); ///=3

// TODO(test): configurable
