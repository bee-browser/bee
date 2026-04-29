try {
const o = Object.defineProperties({}, {
  a: {
    set() {
      return 1;
    },
  },
  b: {
    set: () => 2,
  },
});

// TODO(test): print(o.a); ///=1
// TODO(test): print(o.b); ///=2
} catch (e) {
  print(e.name); ///="InternalError"
}
