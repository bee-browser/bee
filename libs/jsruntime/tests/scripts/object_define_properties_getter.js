try {
const o = Object.defineProperties({}, {
  a: {
    get() {
      return 1;
    },
  },
  b: {
    get: () => 2,
  },
});

// TODO(test): print(o.a); ///=1
// TODO(test): print(o.b); ///=2
} catch (e) {
  print(e.name); ///="InternalError"
}
