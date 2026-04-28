const o = Object.defineProperties({}, {
  a: {
    value: 1,
  },
  b: {
    value: 2,
    enumerable: true,
  },
  c: {
    value: 3,
    enumerable: false,
  },
});

print(o.a); ///=1
print(o.b); ///=2
print(o.c); ///=3

// TODO(test): enumerable
