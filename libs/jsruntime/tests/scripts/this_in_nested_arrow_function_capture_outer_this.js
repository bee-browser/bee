let a = {
  b: function() {
    return (() => {
      return () => this;
    })()();
  },
};

print(a.b() === a); ///=true

const o = {};
print(a.b.call(o) === o); ///=true
