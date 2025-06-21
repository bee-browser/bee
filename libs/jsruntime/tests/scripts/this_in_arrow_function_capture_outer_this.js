let a = {
  b: function () {
    // `this` in this scope will be captured.
    return () => this;
  },
};

print(a.b()() === a); ///=true
