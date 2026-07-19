let a = {
  b: function () {
    // `this` in this scope will be captured.
    return () => this;
  },
};

print(a.b()() === a); ///=true

const o = {};
print(a.b.call(o)() === o); ///=true
print(a.b().call(o) === a); ///=true
