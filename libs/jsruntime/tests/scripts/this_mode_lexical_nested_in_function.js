let a = {
  b: function() {
    return (() => {
      return () => this;
    })()();
  },
};

print(a.b() === a); ///=true
