const o = Object.defineProperties({}, {
  a: {
    value: 1,
  },
  b: {
    value: 2,
    writable: true,
  },
  c: {
    value: 3,
    writable: false,
  },
});

print(o.a); ///=1
print(o.b); ///=2
print(o.c); ///=3

// TODO(test): writable
